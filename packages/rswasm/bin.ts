import { runExit }        from 'clipanion'

import { DevCommand }     from '&compile.ts'
import { ReleaseComamnd } from '&compile.ts'
import { InstallComamnd } from '&install.ts'

await runExit([InstallComamnd, DevCommand, ReleaseComamnd])
