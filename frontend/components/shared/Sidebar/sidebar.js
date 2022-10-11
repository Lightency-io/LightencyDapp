import React, { useContext, useEffect, useRef, useState } from 'react'
import { logoPNG } from '../../../assets/img'
import {
  SDivider,
  SLink,
  SLinkContainer,
  SLinkIcon,
  SLinkLabel,
  SLinkNotification,
  SLogo,
  SScrollable,
  SSearch,
  SSearchIcon,
  SSidebar,
  SSidebarButton,
  SSidebarContainer,
  STheme,
} from './styles'
import { ThemeContext } from '../../../App'
import { useLocation } from 'react-router-dom'

// Icons
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
import { GrStackOverflow } from 'react-icons/gr'
import { MdLogout, MdOutlineOutbond } from 'react-icons/md'
import { RiAdminLine } from 'react-icons/ri'

const Sidebar = ({ isCouncil, sidebarOpen, setSidebarOpen, logout, login }) => {
  const searchRef = useRef(null)
  const { setTheme, theme } = useContext(ThemeContext)
  const { pathname } = useLocation()

  const searchClickHandler = () => {
    if (!sidebarOpen) {
      setSidebarOpen(true)
      searchRef.current.focus()
    } else {
      // search functionality
    }
  }

  return (
    <div>
      <SSidebar style={{ width: sidebarOpen ? `11rem` : `` }}>
        <>
          <SSidebarButton
            isOpen={sidebarOpen}
            onClick={() => setSidebarOpen((p) => !p)}
          >
            <AiOutlineLeft />
          </SSidebarButton>
        </>
        <SLogo>
          <img src={logoPNG} alt="Logo" />
        </SLogo>
        <SSearch
          onClick={searchClickHandler}
          style={!sidebarOpen ? { width: `fit-content` } : {}}
        >
          <SSearchIcon>
            <AiOutlineSearch />
          </SSearchIcon>
          <input
            ref={searchRef}
            placeholder="Search"
            style={!sidebarOpen ? { width: 0, padding: 0 } : {}}
          />
        </SSearch>
        <SDivider />
        <SSidebarContainer>
          <SScrollable>
            {' '}
            {linksArray.map(({ icon, label, notification, to }) => (
              <SLinkContainer
                key={label}
                className="selected"
                isActive={pathname === to}
              >
                <SLink
                  to={to}
                  style={!sidebarOpen ? { width: `fit-content` } : {}}
                >
                  <SLinkIcon>{icon}</SLinkIcon>
                  {sidebarOpen && (
                    <>
                      <SLinkLabel>
                        <p style={{ marginBottom: 0 }}>{label}</p>
                      </SLinkLabel>
                      {/* if notifications are at 0 or null, do not display */}
                      {!!notification && (
                        <SLinkNotification>{notification}</SLinkNotification>
                      )}
                    </>
                  )}
                </SLink>
              </SLinkContainer>
            ))}
            <SDivider />
            {isCouncil ? (
              councilLinksArray.map(({ icon, label, to }) => (
                <>
                  <SLinkContainer key={label}>
                    <SLink
                      to={to}
                      style={!sidebarOpen ? { width: `fit-content` } : {}}
                    >
                      <SLinkIcon>{icon}</SLinkIcon>
                      {sidebarOpen && (
                        <SLinkLabel>
                          {' '}
                          <p style={{ marginBottom: 0 }}>{label}</p>
                        </SLinkLabel>
                      )}
                    </SLink>
                  </SLinkContainer>
                  <SDivider />
                </>
              ))
            ) : (
              <></>
            )}
            {secondaryLinkArray.map(({ icon, label }) => (
              <SLinkContainer key={label}>
                <SLink
                  to="/"
                  style={!sidebarOpen ? { width: `fit-content` } : {}}
                >
                  <SLinkIcon>{icon}</SLinkIcon>
                  {sidebarOpen && (
                    <SLinkLabel>
                      {' '}
                      <p style={{ marginBottom: 0 }}>{label}</p>
                    </SLinkLabel>
                  )}
                </SLink>
              </SLinkContainer>
            ))}
            <SDivider />{' '}
            {/* {thirdLinkArray.map(({ icon, label, to }) => (
          <SLinkContainer key={label}>
            <SLinka
              href={to}
              target={'_blank'}
              style={!sidebarOpen ? { width: `fit-content` } : {}}
            >
              <SLinkIcon>{icon}</SLinkIcon>
              {sidebarOpen && (
                <SLinkLabel>
                  {' '}
                  <p style={{ marginBottom: 0 }}>{label} </p>
                </SLinkLabel>
              )}
            </SLinka>
          </SLinkContainer>
        ))} */}
            <SDivider />
          </SScrollable>
        </SSidebarContainer>
        <STheme>
          {/* {sidebarOpen && <SThemeLabel>Dark Mode</SThemeLabel>}
          <SThemeToggler
            isActive={theme === 'dark'}
            onClick={() => setTheme((p) => (p === 'light' ? 'dark' : 'light'))}
          >
            <SToggleThumb style={theme === 'dark' ? { right: '1px' } : {}} />
          </SThemeToggler> */}
        </STheme>
      </SSidebar>{' '}
    </div>
  )
}

const councilLinksArray = [
  {
    label: 'Council',
    icon: <RiAdminLine />,
    to: '/council',
    notification: 0,
  },
]
const linksArray = [
  {
    label: 'Home',
    icon: <AiOutlineHome />,
    to: '/home',
    notification: 0,
    forCouncil: true,
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

const secondaryLinkArray = [
  {
    label: 'Settings',
    icon: <AiFillSetting />,
  },
  {
    label: 'Logout',
    icon: <MdLogout />,
  },
]

const thirdLinkArray = [
  // {
  //   label: 'Discord',
  //   icon: <SiDiscord />,
  // },
  {
    label: 'Twitter',
    to: 'https://twitter.com/Lightencyio',
    icon: <BsTwitter />,
  },
  {
    label: 'Linkedin',
    to: 'https://www.linkedin.com/company/electrify-network/',
    icon: <AiFillLinkedin />,
  },
  {
    label: 'Website',
    to: 'https://lightency.io/#/',
    icon: <AiOutlineGlobal />,
  },
]
export default Sidebar
