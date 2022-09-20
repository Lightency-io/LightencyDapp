import React, { useState } from 'react'
import { ConnectButton, NavbarContainer, RightContainer } from './style'

//React icons
import { GrConnect } from 'react-icons/gr'
import { Button } from 'primereact/button'
import { GiHamburgerMenu } from 'react-icons/gi'
import { BiX } from 'react-icons/bi'

const Navbar = ({ logout, login }) => {
  const [show, setShow] = useState(false)

  return (
    <NavbarContainer>
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
    </NavbarContainer>
  )
}

export default Navbar
