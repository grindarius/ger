import shell from 'shelljs'

shell.echo(process.cwd())
shell.cp('-r', '../backend/bindings/*', '../frontend/src/types/')
