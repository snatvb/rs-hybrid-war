const path = require('path')
const process = require('process')
const { validate } = require('schema-utils')
const watch = require('node-watch')
const cp = require('child_process')
const fs = require('fs').promises
const fse = require('fs-extra')

const pluginName = 'RsWatchPlugin'

async function moveFile(oldPath, newPath) {
  await fs.mkdir(path.dirname(newPath), { recursive: true })
  return fse.copySync(oldPath, newPath, { overwrite: true })
}

class RsWatchPlugin {
  constructor(options = {}) {
    const src = path.resolve(process.cwd(), 'src')
    options = {
      ...{
        sourceRoot: src,
        crateRoot: process.cwd(),
        mode: 'release',
      },
      ...options,
    }
    validate(require('./options.json'), options, pluginName)
    this.options = options
  }

  execute() {
    const res = cp.spawnSync(
      'wasm-pack',
      ['build', '--target', 'web', '--' + this.options.mode],
      {
        cwd: this.options.crateRoot,
        stdio: 'inherit',
      },
    )

    if (this.options.move) {
      moveFile(path.resolve(this.options.crateRoot, 'pkg'), this.options.move)
    }
    return res
  }

  apply() {
    this.execute()
    watch(this.options.sourceRoot, { recursive: true }, () => {
      this.execute()
    })
  }
}

module.exports = RsWatchPlugin
