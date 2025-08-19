import { ChakraProvider, defaultSystem } from '@chakra-ui/react'
import { createRoot } from 'react-dom/client'
import { App } from '@/App'
import '@/styles/index.scss'

// biome-ignore lint/style/noNonNullAssertion: root 元素在 HTML 中始终存在，断言安全
createRoot(document.getElementById('root')!).render(
    <ChakraProvider value={defaultSystem}>
            <App />
    </ChakraProvider>,
)