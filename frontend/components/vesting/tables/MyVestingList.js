import React, { useState } from 'react'

//Primreact
import { DataTable } from 'primereact/datatable'
import { Column } from 'primereact/column'
import { Button } from 'primereact/button'
import { ConfirmPopup, confirmPopup } from 'primereact/confirmpopup'

//Date format
import moment from 'moment'

const MyVestingList = ({ myVestList }) => {
  const [expandedRows, setExpandedRows] = useState(null)

  // Selected items
  const [selectedVestId, setSelectedVestId] = useState('')

  const accept = () => {
    window.vesting
      .refresh({ v_id: selectedVestId })
      .then((res) => {
        console.log('Sucess', res)
        setSelectedVestId('')
      })
      .catch((err) => {
        console.error(err)
        setSelectedVestId('')
      })
  }

  const reject = () => {
    setSelectedVestId('')
  }

  const confirm1 = (event) => {
    confirmPopup({
      target: event.currentTarget,
      message: 'Are you sure you want to proceed?',
      icon: 'pi pi-exclamation-triangle',
      accept,
      reject,
    })
  }

  const onRowExpand = (event) => {}

  const onRowCollapse = (event) => {}

  const expandAll = () => {
    let _expandedRows = {}
    myVestList.forEach((p) => (_expandedRows[`${p.id}`] = true))

    setExpandedRows(_expandedRows)
  }

  const collapseAll = () => {
    setExpandedRows(null)
  }

  const imageBodyTemplate = (rowData) => {
    return (
      <img
        src="https://res.cloudinary.com/habibii/image/upload/v1664216318/iron_pack_hlf6hm.jpg"
        width={100}
        height={100}
        onError={(e) =>
          (e.target.src =
            'https://www.primefaces.org/wp-content/uploads/2020/05/placeholder.png')
        }
        alt={rowData.image}
        className="product-image"
      />
    )
  }

  const stateBodyTemplate = (rowData) => {
    return rowData.isPaid ? (
      <>
        <i className="pi pi-check-circle" style={{ color: 'green' }}></i>
      </>
    ) : (
      <>
        <i className="pi pi-sync" style={{ color: '#ffde00' }}></i>
      </>
    )
  }

  const idBodyTemplate = (rowData) => {
    return rowData.id.substr(0, 5) + '...' + rowData.id.substr(59, 63)
  }

  const formatTimestampBodyTemplate = (rowData) => {
    let formattedTimestamp = moment(new Date(rowData.timestamp)).calendar()
    return formattedTimestamp
  }

  const actionBodyTemplate = (rowData) => {
    return (
      <>
        <ConfirmPopup />
        <Button
          onClick={(e) => {
            confirm1(e)
            setSelectedVestId(rowData.id)
          }}
          label="Claim"
          className="btn p-button-success mr-2"
          disabled={rowData.locked_amount === 0}
        />
      </>
    )
  }

  const rowExpansionTemplate = (data) => {
    return (
      <div className="orders-subtable">
        <h5>Vesting schedules</h5>
        <DataTable value={data.schedule} responsiveLayout="scroll">
          <Column field="period" header="Period" sortable></Column>
          <Column field="amount" header="Amount" sortable></Column>
          <Column field="payment_date" header="Pay date" sortable></Column>

          <Column
            field="isPaid"
            header="State"
            body={stateBodyTemplate}
          ></Column>
        </DataTable>
      </div>
    )
  }

  const header = (
    <div className="table-header-container">
      <Button
        icon="pi pi-plus"
        label="Expand All"
        onClick={expandAll}
        className="mr-2"
        style={{ marginRight: '1rem' }}
      />
      <Button icon="pi pi-minus" label="Collapse All" onClick={collapseAll} />
    </div>
  )
  return (
    <div className="datatable-rowexpansion-demo">
      <div className="card">
        <DataTable
          value={myVestList}
          expandedRows={expandedRows}
          onRowToggle={(e) => setExpandedRows(e.data)}
          onRowExpand={onRowExpand}
          onRowCollapse={onRowCollapse}
          responsiveLayout="scroll"
          rowExpansionTemplate={rowExpansionTemplate}
          dataKey="id"
          header={header}
        >
          <Column expander style={{ width: '3em' }} />
          <Column header="Id" body={idBodyTemplate} sortable />
          <Column header="Pack" body={imageBodyTemplate} />
          <Column field="amount_of_token" header="Amount bought" sortable />
          <Column field="locked_amount" header="Locked" sortable />
          <Column
            header="Issued at"
            body={formatTimestampBodyTemplate}
            sortable
          />
          <Column body={actionBodyTemplate} />
        </DataTable>
      </div>
    </div>
  )
}

export default MyVestingList
