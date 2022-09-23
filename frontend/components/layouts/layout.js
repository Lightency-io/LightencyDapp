import React from 'react'
import Navbar from '../shared/Navbar/navbar'
import Sidebar from '../shared/Sidebar/sidebar'
import { SLayout, SMain } from './styles'

const Layout = ({ children, show, setShow, login, logout }) => {
  return (
    <SLayout>
      <Sidebar show={show} setShow={setShow} login={login} logout={logout} />
      <SMain>{children}</SMain>
    </SLayout>
  )
}

export default Layout
