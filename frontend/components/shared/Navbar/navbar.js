import React, { useState } from 'react'
import {
  ConnectButton,
  FlexContainer,
  LeftContainer,
  NavbarContainer,
  RightContainer,
} from './style'

import { useMediaQuery } from 'react-responsive'

//React icons
import { GrConnect } from 'react-icons/gr'
import { GiHamburgerMenu } from 'react-icons/gi'
import {
  AiOutlineSearch,
  AiOutlineHome,
  AiOutlineLeft,
  AiFillSetting,
  AiOutlineSwap,
  AiOutlineGlobal,
  AiFillLinkedin,
  AiFillUnlock,
} from 'react-icons/ai'
import { VscOrganization } from 'react-icons/vsc'
import { BsTwitter } from 'react-icons/bs'
import { SiDiscord } from 'react-icons/si '
import { GrStackOverflow } from 'react-icons/gr'
import { MdLogout, MdOutlineOutbond } from 'react-icons/md'
import { useLocation } from 'react-router-dom'

// Primreact
import { Button } from 'primereact/button'
import { Sidebar } from 'primereact/sidebar'
import {
  SDivider,
  SLink,
  SLinkContainer,
  SLinkIcon,
  SLinkLabel,
  SLinkNotification,
  SLogo,
} from '../Sidebar/styles'
import { logoPNG } from '../../../assets/img'

const Navbar = ({ isCouncil, setIsCouncil, logout, login }) => {
  const isTabletOrMobile = useMediaQuery({ query: '(max-width: 1224px)' })
  const [visibleTop, setVisibleTop] = useState(false)
  const { pathname } = useLocation()

  const customIcons = (
    <React.Fragment>
      <button className="p-sidebar-icon p-link mr-1">
        <span className="pi pi-print" />
      </button>
      <button className="p-sidebar-icon p-link mr-1">
        <span className="pi pi-arrow-right" />
      </button>
    </React.Fragment>
  )

  return (
    <FlexContainer>
      {!isTabletOrMobile ? (
        <RightContainer>
          {window.walletConnection.isSignedIn() ? (
            <ConnectButton onClick={logout}>
              <GrConnect style={{ marginRight: '8px' }} />
              {window.accountId}
            </ConnectButton>
          ) : (
            <Button
              className="mt-1"
              style={{ height: '2rem' }}
              label="Connect wallet"
              onClick={login}
            />
          )}
        </RightContainer>
      ) : (
        <>
          <Sidebar
            visible={visibleTop}
            position="top"
            style={{ height: '100vh' }}
            onHide={() => setVisibleTop(false)}
          >
            <SLogo>
              <img src={logoPNG} alt="Logo" />
            </SLogo>
            <SDivider />
            {linksArray.map(({ icon, label, notification, to }) => (
              <SLinkContainer
                key={label}
                className="selected"
                isActive={pathname === to}
              >
                <SLink to={to} style={{ width: `fit-content` }}>
                  <SLinkIcon>{icon}</SLinkIcon>
                  <>
                    <SLinkLabel>
                      <p style={{ marginBottom: 0 }}>{label}</p>
                    </SLinkLabel>
                    {/* if notifications are at 0 or null, do not display */}
                    {!!notification && (
                      <SLinkNotification>{notification}</SLinkNotification>
                    )}
                  </>
                </SLink>
              </SLinkContainer>
            ))}
            <SDivider />
          </Sidebar>
          <LeftContainer>
            <GiHamburgerMenu onClick={() => setVisibleTop(true)} />
          </LeftContainer>
        </>
      )}
    </FlexContainer>
  )
}

const linksArray = [
  {
    label: 'Home',
    icon: <AiOutlineHome />,
    to: '/home',
    notification: 0,
  },
  {
    label: 'Governance',
    icon: <VscOrganization />,
    to: '/governance',
    notification: 0,
  },
  {
    label: 'Swap',
    icon: <AiOutlineSwap />,
    to: '/swap',
    notification: 0,
  },
  {
    label: 'Stake',
    icon: <GrStackOverflow />,
    to: '/stake',
    notification: 0,
  },
  {
    label: 'Bond',
    icon: <MdOutlineOutbond />,
    to: '/bond',
    notification: 0,
  },
  {
    label: 'Vesting',
    icon: <AiFillUnlock />,
    to: '/vesting',
    notification: 0,
  },
]

export default Navbar
