import React, { useEffect, useRef, useState } from 'react'

import styled from 'styled-components'

// Constants
import { VestingPacks } from '../components/vesting/data/vestingPacks'

// Primreact
import { BreadCrumb } from 'primereact/breadcrumb'
import { Toolbar } from 'primereact/toolbar'
import { Button } from 'primereact/button'
import { Dialog } from 'primereact/dialog'
import { DataScroller } from 'primereact/datascroller'
import { Rating } from 'primereact/rating'
import { TabView, TabPanel } from 'primereact/tabview'

import { hashToKeccak256 } from '../utils/keccak256'
import MyVestingList from '../components/vesting/tables/MyVestingList'

// Time formatter
import moment from 'moment'
import VestingList from '../components/vesting/tables/vestingList'

const VestingPage = () => {
  // Lists ( Array of objects)
  const [vestList, setVestList] = useState([])
  const [myVestList, setMyVestList] = useState([])

  const [displayBasic, setDisplayBasic] = useState(false)

  //Selected items
  const [selectedPack, setSelectedPack] = useState('')
  const [isSelectedPack, setIsSelectedPack] = useState(false)

  // Loading
  const [isLoading, setIsLoading] = useState(false)

  const [packs, setPacks] = useState(VestingPacks)
  const ds = useRef(null)

  const itemTemplate = (data) => {
    return (
      <div className="product-item">
        <img
          src={`${data.image}`}
          onError={(e) =>
            (e.target.src =
              'https://www.primefaces.org/wp-content/uploads/2020/05/placeholder.png')
          }
          alt={data.name}
        />
        <div className="product-detail">
          <div className="product-name">{data.name}</div>
          <div className="product-description">{data.description}</div>
          <Rating value={data.rating} readOnly cancel={false}></Rating>
        </div>
        <div className="product-action">
          <span className="product-price">${data.price}</span>
          <Button
            label="Select"
            disabled={data.inventoryStatus === 'OUTOFSTOCK'}
            onClick={() => {
              setSelectedPack(data)
              setIsSelectedPack(true)
              generateId()
            }}
          ></Button>
          <span
            className={`product-badge status-${data.inventoryStatus.toLowerCase()}`}
          >
            {data.inventoryStatus}
          </span>
        </div>
      </div>
    )
  }

  const dialogFuncMap = {
    displayBasic: setDisplayBasic,
  }

  useEffect(() => {
    window.vesting.get_all_vestors().then((list) => {
      let allList = []
      let myList = []
      list.forEach((item) => {
        allList.push({
          ...item,
          account: {
            owner_id: item.owner_id,
            image: 'ionibowcher.png',
          },
        })
        if (item.owner_id === window.accountId.toString())
          myList.push({
            ...item,
            schedule: [
              {
                period: 'First period',
                amount: item.amount_of_token / 4,
                payment_date: moment(new Date(item.timestamp)).calendar(),
                isPaid: item.nb_time_payment >= 1,
              },
              {
                period: 'Second period',
                amount: item.amount_of_token / 4,
                payment_date: moment(new Date(item.timestamp))
                  .add(365, 'days')
                  .calendar(),
                isPaid: item.nb_time_payment >= 2,
              },
              {
                period: 'Third period',
                amount: item.amount_of_token / 4,
                payment_date: moment(new Date(item.timestamp))
                  .add(365 * 2, 'days')
                  .calendar(),
                isPaid: item.nb_time_payment >= 3,
              },
              {
                period: 'Last period',
                amount: item.amount_of_token / 4,
                payment_date: moment(new Date(item.timestamp))
                  .add(365 * 3, 'days')
                  .calendar(),
                isPaid: item.nb_time_payment === 4,
              },
            ],
          })
      })
      console.log(allList)
      setVestList(allList)
      setMyVestList(myList)
    })
  }, [])

  const rightToolbarTemplate = () => {
    return (
      <React.Fragment>
        <Button
          label="Buy"
          style={{ backgroundColor: '#ffde00' }}
          onClick={() => onClick('displayBasic')}
        />
      </React.Fragment>
    )
  }

  const onClick = (name) => {
    dialogFuncMap[`${name}`](true)
  }

  const onHide = (name) => {
    dialogFuncMap[`${name}`](false)
    setSelectedPack({})
    setIsSelectedPack(false)
  }

  const generateId = () => {
    const currentDate = new Date()
    const timestamp = currentDate.getTime()
    let hashId = hashToKeccak256(timestamp + 'test')
    window.vesting
      .add_lockup({ id: hashId.toString(), amount_of_token: 100 })
      .then((res) => {
        console.log('Vestor added successfully', res)
      })
      .catch((err) => {
        console.error('Vestor added failed', err)
      })
  }

  const items = [{ label: 'Vesting' }]

  const home = {
    icon: 'pi pi-home',
    url: '/',
  }
  return (
    <div className="container">
      <div className="row mt-4">
        <div className="col-md-12">
          <Section>
            <BreadCrumb model={items} home={home} />
          </Section>
        </div>
      </div>
      <Section className="mt-4">
        <div className="row">
          <div className="title-container">
            <div className="title">
              <h4>Vesting</h4>
            </div>
          </div>
          <div className="col-md-12 phone">
            <TabView>
              <TabPanel header="All vesting schedules">
                <>
                  {' '}
                  <Toolbar
                    className="mb-4"
                    right={rightToolbarTemplate}
                  ></Toolbar>
                  <VestingList vestList={vestList} />
                </>
              </TabPanel>
              <TabPanel
                header="My vesting schedules"
                disabled={!window.walletConnection.isSignedIn()}
              >
                <>
                  <Toolbar
                    className="mb-4"
                    right={rightToolbarTemplate}
                  ></Toolbar>
                  <MyVestingList myVestList={myVestList} />
                </>
              </TabPanel>
            </TabView>
          </div>
        </div>
      </Section>
      <Dialog
        maximizable
        header="Choose a plan"
        visible={displayBasic}
        style={{ width: '50vw' }}
        onHide={() => onHide('displayBasic')}
      >
        {!isSelectedPack ? (
          <div className="datascroller-demo">
            <div className="card">
              <DataScroller
                ref={ds}
                value={packs}
                itemTemplate={itemTemplate}
                rows={5}
                loader
              />
            </div>
          </div>
        ) : (
          <></>
        )}
      </Dialog>
    </div>
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

  .title-container {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  .title {
    h1 {
      font-size: 2rem;
      letter-spacing: 0.2rem;
    }
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
export default VestingPage
