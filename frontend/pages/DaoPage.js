import { Dialog } from 'primereact/dialog'
import React, { useState, useEffect } from 'react'
import CreateProposal from '../components/dao/createProposal'

//Router
import { Link, useOutlet, useParams } from 'react-router-dom'
import { Outlet, useLocation } from 'react-router-dom'

//Foorm validation
import { useFormik } from 'formik'
import * as Yup from 'yup'

// Swweet alert
import Swal from 'sweetalert2'
import styled from 'styled-components'

//Primreact
import { BreadCrumb } from 'primereact/breadcrumb'

const DaoPage = () => {
  const params = useParams()
  const location = useLocation()

  const [currentDao, setCurrentDao] = useState({})
  const [loader, setLoader] = useState(false)
  const [reload, setReload] = useState('no load')

  const [selectedSubMenu, setSelectedSubMenu] = useState(
    location.pathname.substring(location.pathname.lastIndexOf('/') + 1),
  )

  useEffect(() => {
    console.log(
      location.pathname.substring(location.pathname.lastIndexOf('/') + 1),
    )
    window.contract.get_dao({ dao_name: params.dao_name }).then((dao) => {
      setCurrentDao(dao)
    })
  }, [reload])

  const formik = useFormik({
    initialValues: {
      title: '',
      description: '',
      duration_days: '0',
      duration_hours: '0',
      duration_min: '0',
    },
    validationSchema: Yup.object({
      title: Yup.string()
        .max(30, 'Must be 15 characters or less')
        .required('Title is required'),
      description: Yup.string()
        .max(500, 'Must be 500 characters or less')
        .required('Description is required'),
      duration_days: Yup.number()
        .typeError('Must be a number')
        .min(0, 'Min value is 0')
        .max(365, 'Max value is 365')
        .required('Must be at least 0')
        .integer(),
      duration_hours: Yup.number()
        .typeError('Must be a number')
        .min(0, 'Min value is 0')
        .max(24, 'Max value is 24')
        .required('Must be at least 0')
        .integer(),
      duration_min: Yup.number()
        .typeError('Must be a number')
        .min(0, 'Min value is 0')
        .max(60, 'Max value is 60')
        .required('Must be at least 0')
        .integer(),
    }),
    onSubmit: (data) => {
      setLoader(true)
      window.contract
        .create_proposal({
          dao_name: params.dao_name,
          proposal_name: data.title,
          description: data.description,
          duration_days: parseInt(data.duration_days),
          duration_hours: parseInt(data.duration_hours),
          duration_min: parseInt(data.duration_min),
        })
        .then((res) => {
          Swal.fire({
            position: 'top-end',
            icon: 'success',
            title: 'Your proposal has been added successfully',
            showConfirmButton: false,
            background: 'black',
            iconColor: '#ffde00',
            confirmButtonColor: 'grey',
            timer: 2500,
          })
          onHide('displayBasic2')
          setLoader(false)
          setReload('Load')
        })
        .catch((err) => {
          console.error('Oops something went wrong !', err)
          setLoader(false)
        })
      formik.resetForm()
    },
  })
  const [displayBasic2, setDisplayBasic2] = useState(false)

  const dialogFuncMap = {
    displayBasic2: setDisplayBasic2,
  }

  const onClick = (name) => {
    dialogFuncMap[`${name}`](true)
  }

  const onHide = (name) => {
    dialogFuncMap[`${name}`](false)
  }

  const items = [
    { label: 'Governance', url: '/governance' },
    { label: `${params.dao_name}'s DAO` },
  ]

  const home = {
    icon: 'pi pi-home',
    url: '/',
  }

  return (
    <>
      <div className="container">
        <div className="row mt-4">
          <Section>
            <BreadCrumb model={items} home={home} />
          </Section>
        </div>
        <div className="row mt-4">
          <div className="col-md-12">
            <Section>
              <div id="content" className="content content-full-width">
                <div className="profile">
                  <div className="profile-header">
                    <div className="profile-header-cover"></div>
                    <div className="profile-header-content">
                      <div className="profile-header-img">
                        <img
                          src="https://uploads-ssl.webflow.com/628cf2e86851d26b00615819/628cf2e86851d2bb5d615855_Sun%20circle%201%402x.png"
                          alt=""
                        />
                      </div>

                      <div className="profile-header-info">
                        <h4 className="m-t-10 m-b-5">{currentDao.dao_name}</h4>
                        <p className="m-b-10">{currentDao.dao_purpose}</p>
                        <button
                          className="btn btn-sm btn-info mb-2"
                          style={{ backgroundColor: '#ffde00' }}
                          onClick={() => onClick('displayBasic2')}
                        >
                          Create new proposal
                        </button>
                      </div>
                    </div>
                    <ul className="profile-header-tab nav nav-tabs">
                      <li className="nav-item">
                        <Link
                          to=""
                          className={
                            selectedSubMenu === 'landing'
                              ? 'nav-link_ active'
                              : 'nav-link_'
                          }
                          onClick={() => setSelectedSubMenu('landing')}
                        >
                          Home
                        </Link>
                      </li>
                      <li className="nav-item">
                        <Link
                          to="proposals"
                          className={
                            selectedSubMenu === 'proposals'
                              ? 'nav-link_ active'
                              : 'nav-link_'
                          }
                          onClick={() => setSelectedSubMenu('proposals')}
                        >
                          Proposals
                        </Link>
                      </li>
                      <li className="nav-item">
                        <Link to="funds" className="nav-link_">
                          Funds
                        </Link>
                      </li>
                      <li className="nav-item">
                        <Link to="members" className="nav-link_">
                          Members
                        </Link>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
            </Section>
          </div>
        </div>
        <Outlet context={[reload, setReload]} />
        <Dialog
          header={loader ? 'Please Wait' : 'Create proposal'}
          visible={displayBasic2}
          className="dialog"
          onHide={() => onHide('displayBasic2')}
        >
          <CreateProposal formik={formik} onHide={onHide} loader={loader} />
        </Dialog>
      </div>
    </>
  )
}

const Section = styled.section`
  background-color: black;
  border-radius: 1rem;
  padding: 1rem;
  height: 100%;
  width: 100%;

  @media only screen and (max-width: 550px) {
    width: 22rem;
  }

  .p-menuitem-link {
    :hover {
      .p-menuitem-text {
        color: #ffde00;
      }
      .p-menuitem-icon {
        color: #ffde00;
      }
    }
  }
`

export default DaoPage
