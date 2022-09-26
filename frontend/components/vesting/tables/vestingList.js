import { Column } from 'primereact/column'
import { DataTable } from 'primereact/datatable'
import React from 'react'

const VestingList = ({ vestList }) => {
  const headerTemplate = (data) => {
    return (
      <React.Fragment>
        <img
          alt={data.account.owner_id}
          src="https://res.cloudinary.com/habibii/image/upload/v1664232802/ffa09aec412db3f54deadf1b3781de2a_vchuvq.png"
          width="32"
          style={{ verticalAlign: 'middle', paddingRight: '10px' }}
        />
        <span className="image-text">{data.account.owner_id}</span>
      </React.Fragment>
    )
  }
  const footerTemplate = (data) => {
    return (
      <React.Fragment>
        <td colSpan="4" style={{ textAlign: 'right' }}>
          Total Vesting
        </td>
        <td>{calculateVestingTotal(data.account.owner_id)}</td>
      </React.Fragment>
    )
  }

  const calculateVestingTotal = (owner_id) => {
    let total = 0

    if (vestList) {
      for (let vesting of vestList) {
        if (vesting.account.owner_id === owner_id) {
          total++
        }
      }
    }

    return total
  }
  return (
    <div>
      {' '}
      <DataTable
        value={vestList}
        rowGroupMode="subheader"
        groupRowsBy="account.owner_id"
        sortMode="single"
        sortField="account.owner_id"
        sortOrder={1}
        scrollable
        scrollHeight="400px"
        rowGroupHeaderTemplate={headerTemplate}
        rowGroupFooterTemplate={footerTemplate}
        responsiveLayout="scroll"
      >
        <Column
          field="amount_of_token"
          header="Amount bought"
          style={{ minWidth: '200px' }}
          sortable
        ></Column>
        <Column
          field="locked_amount"
          header="Locked"
          style={{ minWidth: '200px' }}
          sortable
        ></Column>
        <Column
          field="unlocked_amount"
          header="Unlocked"
          style={{ minWidth: '200px' }}
          sortable
        ></Column>
      </DataTable>
    </div>
  )
}

export default VestingList
