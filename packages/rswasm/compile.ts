#!/usr/bin/env bun

import { join }    from 'node:path'

import { Command } from 'clipanion'
import { parse }   from 'smol-toml'

const DIR = join(import.meta.dir, '..')
const TARGET_DIR = join(DIR, '..', '..', 'target', 'wasm32-unknown-unknown')
const CRATE_DIR = process.cwd()
const CARGO_TOML = join(CRATE_DIR, 'Cargo.toml')
const OUT_DUR = join(CRATE_DIR, 'dist')
const TOOLCHAIN_DIR = join(DIR, 'toolchain')

const EXE_EXT = process.platform === 'win32' ? '.exe' : ''
const TOOLCHAIN_BIN_TARGET = `-${process.platform}-${process.arch}${EXE_EXT}`

const WASM_BINDGEN_BIN = 'wasm-bindgen' + TOOLCHAIN_BIN_TARGET
const WASM_OPT_BIN = 'wasm-opt' + TOOLCHAIN_BIN_TARGET

type CargoToml = {
	package: {
		name: string
	}
}

const cargoToml = await Bun.file(CARGO_TOML).text()
const {
	package: { name: CRATE_NAME },
} = parse(cargoToml) as CargoToml

const DEBUG_ARTIFACT = join(TARGET_DIR, 'debug', `${CRATE_NAME}.wasm`)
const RELEASE_ARTIFACT = join(TARGET_DIR, 'release', `${CRATE_NAME}.wasm`)

const { $ } = Bun

const COMMON_OPTS = ['--split-linked-modules', '--target', 'web', '--out-dir', OUT_DUR, '--out-name', 'crate']

export class DevCommand extends Command {
	static paths = [['dev']]

	async execute() {
		await $`cargo build --target wasm32-unknown-unknown`
		await $`./${WASM_BINDGEN_BIN} ${COMMON_OPTS} ${DEBUG_ARTIFACT}`.cwd(TOOLCHAIN_DIR)
	}
}

export class ReleaseComamnd extends Command {
	static paths = [['release']]

	async execute() {}
}
