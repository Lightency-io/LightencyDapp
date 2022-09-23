import { createGlobalStyle } from 'styled-components'
import { wallpaperDarkJPG } from '../assets/img'

export const GlobalStyle = createGlobalStyle`
    *, *::before, *::after {
        box-sizing: border-box;
    }

    html {
        scroll-behavior: smooth;
        height: 100%
      }
      
    body {
        background: ${({ theme }) => theme.bg2};
        background-image: url(${wallpaperDarkJPG});
        color: ${({ theme }) => theme.text};
        font-family: 'Roboto', sans-serif;
        letter-spacing: .6px;
        background-size: cover;
        background-repeat: no-repeat;
        background-attachment: fixed;
        height: max-content;
        overflow-x: hidden;
        
    }

    ::-webkit-scrollbar {
    }
    
    ::-webkit-scrollbar-thumb {
    }
`
