import styled from "styled-components";

export const NavbarContainer = styled.nav`
  width: 100%;
  height: 60px;
  background-color: black;
  display: flex;
  flex-direction: column;

  position: absolute;
`;

export const RightContainer = styled.div`
  flex: 30%;
  display: flex;
  justify-content: flex-end;
  padding-right: 50px;
  background-color: salamon;
`;

export const ConnectButton = styled.button`
  margin-top: 0.4rem;
  width: 10rem;
  height: 3rem;

  background-color: #ffde00;
  border-radius: 9px;
  color: white;

  border: none;
  font-weight: bold;
`;

export const TranslateButton = styled.button`
  margin-top: 0.2rem;
`;
