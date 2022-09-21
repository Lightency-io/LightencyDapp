import React, { useState } from "react";
import { ConnectButton, NavbarContainer, RightContainer } from "./style";

//React icons
import { GrConnect } from "react-icons/gr";
import { Button } from "primereact/button";
import { GiHamburgerMenu } from "react-icons/gi";
import { BiX } from "react-icons/bi";

const Navbar = ({ logout, login }) => {
  const [show, setShow] = useState(false);

  return (
    <NavbarContainer>
      <RightContainer>
        <div>
          <p style={{padding : 20}}>{window.accountId}</p>
        </div>
        {window.walletConnection.isSignedIn() ? (
          <ConnectButton onClick={logout}>logout</ConnectButton>
        ) : (
          <ConnectButton onClick={login}> Connect </ConnectButton>
        )}
      </RightContainer>
    </NavbarContainer>
  );
};

export default Navbar;
