import React, { useEffect, useState } from 'react'
import styled from 'styled-components'
import Chart1 from '../components/governance/charts/chart1'
import CreateDao from '../components/governance/forms/createDao'
import DaoList from '../components/governance/tables/daoList'
import Loading from '../components/shared/Loading/loading'
import { login } from '../utils'

//Primreact
import { BreadCrumb } from 'primereact/breadcrumb'

//Form validation
import { useFormik } from 'formik'
import * as Yup from 'yup'
import Swal from 'sweetalert2'

import scrollreveal from 'scrollreveal'

const GovernancePage = () => {
  const [reload, setReload] = useState('no load')

  //Loading variables
  const [isLoading, setIsLoading] = useState(false)

  const [activeIndex, setActiveIndex] = useState(0)

  const goNext = () => {
    setActiveIndex(activeIndex + 1)
  }

  const goPrevious = () => {
    setActiveIndex(activeIndex - 1)
  }

  const formik = useFormik({
    initialValues: {
      name: '',
      purpose: '',
      daysDuration: '0',
      hoursDuration: '0',
      minuteDuration: '0',
    },
    validationSchema: Yup.object({
      name: Yup.string()
        .max(15, 'Must be 15 characters or less')
        .required('Name is required'),
      purpose: Yup.string()
        .max(250, 'Must be 15 characters or less')
        .required('Purpose is required'),
      daysDuration: Yup.number()
        .typeError('Must be a number')
        .min(0, 'Min value is 0')
        .max(365, 'Max value is 365')
        .required('Must be at least 0')
        .integer(),
      hoursDuration: Yup.number()
        .typeError('Must be a number')
        .min(0, 'Min value is 0')
        .max(24, 'Max value is 24')
        .required('Must be at least 0')
        .integer(),
      minuteDuration: Yup.number()
        .typeError('Must be a number')
        .min(0, 'Min value is 0')
        .max(60, 'Max value is 60')
        .required('Must be at least 0')
        .integer(),
    }),
    onSubmit: (data) => {
      if (window.walletConnection.isSignedIn()) {
        setIsLoading(true)
        window.contract
          .add_dao({
            dao_name: data.name,
            dao_purpose: data.purpose,
            threshold: 0,
            duration_days: parseInt(data.daysDuration),
            duration_hours: parseInt(data.hoursDuration),
            duration_min: parseInt(data.minuteDuration),
          })
          .then(() => {
            Swal.fire({
              position: 'top-end',
              icon: 'success',
              title: 'Your DAO has been added successfully',
              showConfirmButton: false,
              background: 'black',
              iconColor: '#ffde00',
              confirmButtonColor: 'grey',
              timer: 2500,
            })
            setIsLoading(false)
            setReload('Load')
          })
          .catch((err) => {
            setIsLoading(false)
            console.error('Oops something went wrong !', err)
          })
        setActiveIndex(0)
        formik.resetForm()
      } else {
        login()
      }
    },
  })

  useEffect(() => {
    const sr = scrollreveal({
      origin: 'left',
      distance: '80px',
      duration: 1000,
      reset: false,
    })
    sr.reveal(
      `
       #governance
    `,
      { easing: 'ease-in' },
    )
  }, [])

  const items = [{ label: 'Governance', url: '/governance' }]

  const home = {
    icon: 'pi pi-home',
    url: '/',
  }

  return (
    <>
      <div className="container mt-4" id="governance">
        <div className="row">
          <div className="col-md-12">
            <Section>
              <BreadCrumb model={items} home={home} />
            </Section>
          </div>
        </div>
        <div className="row mt-4">
          <div className="col-md-8">
            <Section>
              <Chart1 />
            </Section>
          </div>
          <div className="col-md-4 phone">
            <Section>
              <div style={{ paddingLeft: '28%', paddingTop: '5%' }}>
                <Loading />
              </div>
            </Section>
          </div>
        </div>
        <div className="row mt-4">
          <div className="col-md-8">
            <Section>
              <CreateDao
                formik={formik}
                isLoading={isLoading}
                activeIndex={activeIndex}
                setActiveIndex={setActiveIndex}
                goNext={goNext}
                goPrevious={goPrevious}
              />
            </Section>
          </div>
          <div className="col-md-4 phone">
            <Section>
              <div style={{ paddingLeft: '28%', paddingTop: '5%' }}>
                <Loading />
              </div>
            </Section>
          </div>
        </div>
        <div className="row mt-4">
          <div className="col-md-12">
            <Section>
              <DaoList reload={reload} setReload={setReload} />
            </Section>
          </div>
        </div>
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

export default GovernancePage
