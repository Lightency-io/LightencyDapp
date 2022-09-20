import React, { useContext, useRef, useState } from 'react'
import { logoPNG } from '../../../assets/img'
import {
  SDivider,
  SLink,
  SLinka,
  SLinkContainer,
  SLinkIcon,
  SLinkLabel,
  SLinkNotification,
  SLogo,
  SSearch,
  SSearchIcon,
  SSidebar,
  SSidebarButton,
  STheme,
  SThemeLabel,
  SThemeToggler,
  SToggleThumb,
} from './styles'
import { ThemeContext } from '../../../App'
import { Link, useLocation } from 'react-router-dom'

// Icons
import {
  AiOutlineSearch,
  AiOutlineHome,
  AiOutlineLeft,
  AiFillSetting,
  AiOutlineSwap,
  AiFillLinkedin,
  AiOutlineGlobal,
} from 'react-icons/ai'
import { VscOrganization } from 'react-icons/vsc'
import { GrStackOverflow } from 'react-icons/gr'
import { MdLogout, MdOutlineOutbond } from 'react-icons/md'
import { BsTwitter } from 'react-icons/bs'
import { GiHamburgerMenu } from 'react-icons/gi'
import { BiX } from 'react-icons/bi'
import Navbar from '../Navbar/navbar'
import { login, logout } from '../../../utils'

const Sidebar = () => {
  const searchRef = useRef(null)
  const { setTheme, theme } = useContext(ThemeContext)
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const { pathname } = useLocation()
  const [show, setShow] = useState(false)

  const searchClickHandler = () => {
    if (!show) {
      setShow(true)
      searchRef.current.focus()
    } else {
      // search functionality
    }
  }
  return (
    <main className={show ? 'main space-toggle' : 'main'}>
      <header className={`header ${show ? 'space-toggle' : null}`}>
        <div className="header-toggle" onClick={() => setShow(!show)}>
          {!show ? <GiHamburgerMenu /> : <BiX />}
        </div>
      </header>
      <aside className={`sidebar ${show ? 'show' : null}`}>
        <nav className="navbar">
          <Link to="/" className="nav-logo">
            <SLogo>
              <img src={logoPNG} alt="Logo" />
            </SLogo>{' '}
            {/* here comes the Lightency title  */}
          </Link>
          <SSearch
            onClick={() => searchClickHandler}
            style={!show ? { width: `fit-content` } : {}}
          >
            <SSearchIcon>
              <AiOutlineSearch />
            </SSearchIcon>
            <input
              ref={searchRef}
              placeholder="Search"
              style={!show ? { width: 0, padding: 0 } : {}}
            />
          </SSearch>
          <SDivider />
          {linksArray.map(({ icon, label, notification, to }) => (
            <SLinkContainer
              key={label}
              className="selected"
              isActive={pathname === to}
            >
              <SLink to={to}>
                <SLinkIcon>{icon}</SLinkIcon>
                {show && (
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
          {secondaryLinkArray.map(({ icon, label }) => (
            <SLinkContainer
              key={label}
              style={show ? { width: `fit-content` } : {}}
            >
              <SLink to="/" style={!show ? { width: `fit-content` } : {}}>
                <SLinkIcon>{icon}</SLinkIcon>
                {show && (
                  <SLinkLabel>
                    {' '}
                    <p style={{ marginBottom: 0 }}>{label}</p>
                  </SLinkLabel>
                )}
              </SLink>
            </SLinkContainer>
          ))}
          <SDivider />{' '}
          {thirdLinkArray.map(({ icon, label, to }) => (
            <SLinkContainer
              key={label}
              style={!show ? { width: `fit-content` } : {}}
            >
              <SLinka
                href={to}
                target={'_blank'}
                style={!show ? { width: `fit-content` } : {}}
              >
                <SLinkIcon>{icon}</SLinkIcon>
                {show && (
                  <SLinkLabel>
                    {' '}
                    <p style={{ marginBottom: 0 }}>{label} </p>
                  </SLinkLabel>
                )}
              </SLinka>
            </SLinkContainer>
          ))}
          <SDivider />
          {/* <STheme>
            {show && <SThemeLabel>Dark Mode</SThemeLabel>}
            <SThemeToggler
              isActive={theme === 'dark'}
              onClick={() =>
                setTheme((p) => (p === 'light' ? 'dark' : 'light'))
              }
            >
              <SToggleThumb style={theme === 'dark' ? { right: '1px' } : {}} />
            </SThemeToggler>
          </STheme> */}
        </nav>
      </aside>
    </main>
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
