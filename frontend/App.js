import 'regenerator-runtime/runtime'
import React, { useState } from 'react'

import './assets/css/global.css'

import { login, logout, get_greeting, set_greeting } from './utils'
import getConfig from './config'
import { darkTheme, lightTheme } from './styles/theme'
import { GlobalStyle } from './styles/globalStyles'
import { Helmet } from 'react-helmet'
import styled, { ThemeProvider } from 'styled-components'
import Navbar from './components/shared/Navbar/navbar'
import Layout from './components/layouts/layout'
import Swal from 'sweetalert2'

//Routing
import Routes from './Routes'

import OnDevelopmentModPage from './pages/OnDevelopmentModPage'
import Sidebar from './components/shared/Sidebar/sidebar'
import Footer from './components/shared/Footer/footer'
import { isWhitelisted } from './utils/isWhitelisted'

export const ThemeContext = React.createContext(null)

export default function App() {
  // use React Hooks to store greeting in component state
  const [greeting, setGreeting] = React.useState()
  const [isCouncil, setIsCouncil] = useState(false)
  const [theme, setTheme] = useState('light')
  const themeStyle = theme === 'light' ? lightTheme : darkTheme

  const [sidebarOpen, setSidebarOpen] = useState(false)

  // when the user has not yet interacted with the form, disable the button
  const [buttonDisabled, setButtonDisabled] = React.useState(true)

  // after submitting the form, we want to show Notification
  const [showNotification, setShowNotification] = React.useState(false)

  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    () => {
      setIsCouncil(isWhitelisted(window.accountId))
      // get_greeting is in near/utils.js
      Swal.fire({
        title: 'Still on development Mode !',
        text:
          'Our platform is still under construction, however we left access to the potential users to check it. Please keep in mind when you see the logo of Lightency refilling , that means it is under development',
        icon: 'warning',
        confirmButtonText: 'Cool',
        background: 'black',
        iconColor: '#ffde00',
        confirmButtonColor: 'grey',
      })
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render"
    // This works because signing into NEAR Wallet reloads the page
    [],
  )

  // if not signed in, return early with sign-in prompt
  // if (!window.walletConnection.isSignedIn()) {
  //   return (
  //     <ThemeContext.Provider value={{ setTheme, theme }}>
  //       <ThemeProvider theme={themeStyle}>
  //         <GlobalStyle />
  //         <OnDevelopmentModPage login={login} />
  //       </ThemeProvider>
  //     </ThemeContext.Provider>
  //   )
  // }

  return (
    <>
      <ThemeContext.Provider value={{ setTheme, theme }}>
        <ThemeProvider theme={themeStyle}>
          <GlobalStyle />
          <Helmet>
            <title>Lightency platform - DAO protocol</title>
          </Helmet>
          <Wrapper>
            <NavbarContainer>
              <Navbar
                isCouncil={isCouncil}
                setIsCouncil={setIsCouncil}
                logout={logout}
                login={login}
              />
            </NavbarContainer>
            <MainContainer>
              <Routes />
            </MainContainer>
            <SidebarContainer>
              <Sidebar
                isCouncil={isCouncil}
                sidebarOpen={sidebarOpen}
                setSidebarOpen={setSidebarOpen}
                logout={logout}
                login={login}
              />
            </SidebarContainer>
            <FooterContainer className="mt-4">
              <Footer />
            </FooterContainer>
          </Wrapper>
        </ThemeProvider>
      </ThemeContext.Provider>
    </>
  )
}

// this component gets rendered by App after the form is submitted
function Notification() {
  const { networkId } = getConfig(process.env.NODE_ENV || 'development')
  const urlPrefix = `https://explorer.${networkId}.near.org/accounts`

  return (
    <aside>
      <a
        target="_blank"
        rel="noreferrer"
        href={`${urlPrefix}/${window.accountId}`}
      >
        {window.accountId}
      </a>
      {
        ' ' /* React trims whitespace around tags; insert literal space character when needed */
      }
      called method: 'set_greeting' in contract:{' '}
      <a
        target="_blank"
        rel="noreferrer"
        href={`${urlPrefix}/${window.contract.contractId}`}
      >
        {window.contract.contractId}
      </a>
      <footer>
        <div>✔ Succeeded</div>
        <div>Just now</div>
      </footer>
    </aside>
  )
}

export const Wrapper = styled.div`
  display: grid;
  height: 100vh;
  grid-template-columns: 0.2fr 1.5fr 1fr 1fr;
  grid-template-rows: 0.2fr 3.4fr 0.1fr;
  grid-template-areas:
    'sidebar nav nav nav'
    'sidebar main main main'
    'sidebar footer footer footer';

  @media only screen and (max-width: 550px) {
    grid-template-columns: 1fr;
    grid-template-rows: 0.5fr 6.9fr 0.2fr;
    grid-template-areas:
      'nav'
      'main'
      'footer';
  }
`

export const NavbarContainer = styled.nav`
  grid-area: nav;
  background-color: black;
  opacity: 40%;

  @media only screen and (max-width: 550px) {
    height: 1.5rem;
  }
`

export const MainContainer = styled.main`
  grid-area: main;
  height: 100%;
  overflow-y: scroll;
`

export const SidebarContainer = styled.div`
  grid-area: sidebar;
  @media only screen and (max-width: 550px) {
    display: none;
  }
`

export const FooterContainer = styled.div`
  background: black;
  grid-area: footer;
`
