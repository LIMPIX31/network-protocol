import { createRoot }    from 'react-dom/client'

import { withProviders } from './providers.tsx'
import { Root }          from './root.tsx'

const RootWithProviders = withProviders(Root)

createRoot(document.getElementById('root')!).render(<RootWithProviders />)
