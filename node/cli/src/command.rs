// Copyright 2022 Capsule Corp (France) SAS.
// This file is part of Ternoa.

// Ternoa is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Ternoa is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Ternoa.  If not, see <http://www.gnu.org/licenses/>.

use crate::cli::{Cli, Subcommand};
use frame_benchmarking_cli::BenchmarkCmd;
use node_inspect::cli::InspectCmd;
use sc_cli::{
	BuildSpecCmd, ChainInfoCmd, ChainSpec, CheckBlockCmd, ExportBlocksCmd, ExportStateCmd,
	ImportBlocksCmd, PurgeChainCmd, Result, RevertCmd, RuntimeVersion, SubstrateCli,
};
use sc_service::{Arc, PartialComponents};
use ternoa_client::benchmarking::{inherent_benchmark_data, RemarkBuilder};
use ternoa_service::{chain_spec, new_full, new_partial, IdentifyVariant};

#[cfg(feature = "alphanet-native")]
use ternoa_service::alphanet_runtime;
#[cfg(feature = "alphanet-native")]
use ternoa_service::AlphanetExecutorDispatch;

#[cfg(feature = "mainnet-native")]
use ternoa_service::mainnet_runtime;
#[cfg(feature = "mainnet-native")]
use ternoa_service::MainnetExecutorDispatch;
use try_runtime_cli::TryRuntimeCmd;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Ternoa Node".into()
	}

	fn impl_version() -> String {
		env!("CARGO_PKG_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/capsule-corp-ternoa/chain/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"alphanet" => Box::new(chain_spec::alphanet_config()?),
			#[cfg(feature = "alphanet-native")]
			"alphanet-dev" | "a-dev" | "dev" => Box::new(chain_spec::alphanet::development_config()),

			"mainnet" => Box::new(chain_spec::mainnet_config()?),
			#[cfg(feature = "mainnet-native")]
			"mainnet-dev" | "m-dev" => Box::new(chain_spec::mainnet::development_config()),

			"" => return Err("Please specify which chain you want to run!".into()),
			path => {
				let path = std::path::PathBuf::from(path);

				let chain_spec =
					Box::new(chain_spec::MainnetChainSpec::from_json_file(path.clone())?)
						as Box<dyn sc_service::ChainSpec>;

				if chain_spec.is_alphanet() {
					Box::new(chain_spec::AlphanetChainSpec::from_json_file(path)?)
				} else {
					chain_spec
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		#[cfg(feature = "alphanet-native")]
		if spec.is_alphanet() {
			return &alphanet_runtime::VERSION
		}

		#[cfg(feature = "mainnet-native")]
		{
			return &mainnet_runtime::VERSION
		}

		#[cfg(not(feature = "mainnet-native"))]
		panic!("No runtime feature (alphanet, mainnet) is enabled");
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	// When we call cli.create_runner() it automatically calls the cli.load_spec() function. The
	// loaded spec is stored inside runner.config().chain_spec.

	match &cli.subcommand {
		None => run_wo_args(&cli),
		Some(Subcommand::BuildSpec(cmd)) => build_spec(&cli, cmd),
		Some(Subcommand::CheckBlock(cmd)) => check_block(&cli, cmd),
		Some(Subcommand::ExportBlocks(cmd)) => export_blocks(&cli, cmd),
		Some(Subcommand::ExportState(cmd)) => export_state(&cli, cmd),
		Some(Subcommand::ImportBlocks(cmd)) => import_blocks(&cli, cmd),
		Some(Subcommand::PurgeChain(cmd)) => purge_chain(&cli, cmd),
		Some(Subcommand::Revert(cmd)) => revert(&cli, cmd),
		Some(Subcommand::Benchmark(cmd)) => benchmark(&cli, cmd),
		Some(Subcommand::Inspect(cmd)) => inspect(&cli, cmd),
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => try_runtime(&cli, cmd),
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
							 You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
		Some(Subcommand::Verify(cmd)) => Ok(cmd.run()?),
		Some(Subcommand::Vanity(cmd)) => Ok(cmd.run()?),
		Some(Subcommand::Sign(cmd)) => Ok(cmd.run()?),
		Some(Subcommand::ChainInfo(cmd)) => chain_info(&cli, cmd),
	}?;

	Ok(())
}

fn ensure_dev(spec: &Box<dyn sc_service::ChainSpec>) -> Result<()> {
	if spec.is_dev() {
		Ok(())
	} else {
		panic!("Only Dev Specification Allowed!")
	}
}

macro_rules! with_runtime {
	($chain_spec:expr, $code:expr) => {
		#[cfg(feature = "alphanet-native")]
		if $chain_spec.is_alphanet() {
			#[allow(unused_imports)]
			use alphanet_runtime::Block;
			#[allow(unused_imports)]
			use alphanet_runtime::RuntimeApi;
			#[allow(unused_imports)]
			use AlphanetExecutorDispatch as ExecutorDispatch;

			return $code
		}

		#[cfg(feature = "mainnet-native")]
		{
			#[allow(unused_imports)]
			use mainnet_runtime::Block;
			#[allow(unused_imports)]
			use mainnet_runtime::RuntimeApi;
			#[allow(unused_imports)]
			use MainnetExecutorDispatch as ExecutorDispatch;

			return $code
		}

		#[cfg(not(feature = "mainnet-native"))]
		panic!("No runtime feature (alphanet, mainnet) is enabled");
	};
}

fn run_wo_args(cli: &Cli) -> Result<()> {
	let runner = cli.create_runner(&cli.run)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.run_node_until_exit(|config| async move {
			new_full::<RuntimeApi, ExecutorDispatch>(config).map_err(sc_cli::Error::Service)
		})
	});
}

fn build_spec(cli: &Cli, cmd: &BuildSpecCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	Ok(runner.sync_run(|config| cmd.run(config.chain_spec, config.network))?)
}

fn check_block(cli: &Cli, cmd: &CheckBlockCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.async_run(|config| {
			let PartialComponents { client, task_manager, import_queue, .. } =
				new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
			Ok((cmd.run(client, import_queue), task_manager))
		})
	});
}

fn export_blocks(cli: &Cli, cmd: &ExportBlocksCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.async_run(|config| {
			let PartialComponents { client, task_manager, .. } =
				new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
			Ok((cmd.run(client, config.database), task_manager))
		})
	});
}

fn export_state(cli: &Cli, cmd: &ExportStateCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.async_run(|config| {
			let PartialComponents { client, task_manager, .. } =
				new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
			Ok((cmd.run(client, config.chain_spec), task_manager))
		})
	});
}

fn import_blocks(cli: &Cli, cmd: &ImportBlocksCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.async_run(|config| {
			let PartialComponents { client, task_manager, import_queue, .. } =
				new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
			Ok((cmd.run(client, import_queue), task_manager))
		})
	});
}

fn purge_chain(cli: &Cli, cmd: &PurgeChainCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	Ok(runner.sync_run(|config| cmd.run(config.database))?)
}

fn revert(cli: &Cli, cmd: &RevertCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.async_run(|config| {
			let PartialComponents { client, task_manager, backend, .. } =
				new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;

			let aux_revert = Box::new(|client, _, blocks| {
				sc_finality_grandpa::revert(client, blocks)?;
				Ok(())
			});
			Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
		})
	});
}

fn benchmark(cli: &Cli, cmd: &BenchmarkCmd) -> Result<()> {
	if !cfg!(feature = "runtime-benchmarks") {
		return Err("Benchmarking wasn't enabled when building the node. \
					 You can enable it with `--features runtime-benchmarks`."
			.into())
	}

	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	match cmd {
		BenchmarkCmd::Pallet(cmd) => {
			ensure_dev(chain_spec)?;
			with_runtime!(chain_spec, {
				runner.sync_run(|config| cmd.run::<Block, ExecutorDispatch>(config))
			});
		},
		#[cfg(not(feature = "runtime-benchmarks"))]
		BenchmarkCmd::Storage(_) =>
			Err("Storage benchmarking can be enabled with `--features runtime-benchmarks`.".into()),
		#[cfg(feature = "runtime-benchmarks")]
		BenchmarkCmd::Storage(_) => {
			todo!()
			// with_runtime!(chain_spec, {
			// 	runner.sync_run(|config| {
			// 		// ensure that we keep the task manager alive
			// 		let partial = new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
			// 		let db = partial.backend.expose_db();
			// 		let storage = partial.backend.expose_storage();

			// 		cmd.run(config, partial.client, db, storage)
			// 	})
			// });
		},
		BenchmarkCmd::Overhead(cmd) => {
			#[cfg(feature = "alphanet-native")]
			if chain_spec.is_alphanet() {
				return runner.sync_run(|config| {
					let PartialComponents { client, .. } = new_partial::<
						alphanet_runtime::RuntimeApi,
						AlphanetExecutorDispatch,
					>(&config)?;

					// We need to create Arc<Client> out of the raw client.

					let new_client = ternoa_client::Client::Alphanet(client.clone());
					let arc_client = Arc::new(new_client);

					let builder = RemarkBuilder::new(arc_client.clone());
					cmd.run(
						config,
						client.clone(),
						inherent_benchmark_data().unwrap(),
						Vec::new(),
						&builder,
					)
				})
			}

			#[cfg(feature = "mainnet-native")]
			{
				return runner.sync_run(|config| {
					let PartialComponents { client, .. } = new_partial::<
						mainnet_runtime::RuntimeApi,
						MainnetExecutorDispatch,
					>(&config)?;

					// We need to create Arc<Client> out of the raw client.

					let new_client = ternoa_client::Client::Mainnet(client.clone());
					let arc_client = Arc::new(new_client);

					let builder = RemarkBuilder::new(arc_client.clone());
					cmd.run(
						config,
						client.clone(),
						inherent_benchmark_data().unwrap(),
						Vec::new(),
						&builder,
					)
				})
			}
		},
		BenchmarkCmd::Block(cmd) => {
			with_runtime!(chain_spec, {
				runner.sync_run(|config| {
					let partial = new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
					cmd.run(partial.client)
				})
			});
		},
		BenchmarkCmd::Extrinsic(_cmd) => todo!(),
		_ => panic!("Benchmark Command not implement."),
	}
}

fn inspect(cli: &Cli, cmd: &InspectCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.sync_run(|config| cmd.run::<Block, RuntimeApi, ExecutorDispatch>(config))
	});
}

#[allow(dead_code)]
fn try_runtime(cli: &Cli, cmd: &TryRuntimeCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();
	ensure_dev(chain_spec)?;

	with_runtime!(chain_spec, {
		runner.async_run(|config| {
			// only need a runtime or a task manager to do `async_run`.
			let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
			let task_manager = sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
				.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

			Ok((cmd.run::<Block, ExecutorDispatch>(config), task_manager))
		})
	});
}

fn chain_info(cli: &Cli, cmd: &ChainInfoCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	with_runtime!(chain_spec, {
		runner.run_node_until_exit(|config| async move {
			new_full::<RuntimeApi, ExecutorDispatch>(config).map_err(sc_cli::Error::Service)
		})
	});
}

//
//
//
//
