import React, { useEffect, useState } from 'react'
import styled from 'styled-components'

//Primreact
import { FilterMatchMode, FilterOperator } from 'primereact/api'
import { Toolbar } from 'primereact/toolbar'
import { Dialog } from 'primereact/dialog'
import { Button } from 'primereact/button'
import { DataTable } from 'primereact/datatable'
import { InputText } from 'primereact/inputtext'
import { Column } from 'primereact/column'

const CouncilPage = () => {
  const [displayBasic, setDisplayBasic] = useState(false)

  // List of proposals
  const [proposals, setProposals] = useState([])

  // Filters
  const [filters1, setFilters1] = useState({
    global: { value: null, matchMode: FilterMatchMode.CONTAINS },
    name: {
      operator: FilterOperator.AND,
      constraints: [{ value: null, matchMode: FilterMatchMode.STARTS_WITH }],
    },
  })

  const dialogFuncMap = {
    displayBasic: setDisplayBasic,
  }

  const rightToolbarTemplate = () => {
    return (
      <React.Fragment>
        <Button
          label="Create proposal"
          icon="pi pi-upload"
          style={{ backgroundColor: '#ffde00' }}
          className="p-button-help"
          onClick={() => onClick('displayBasic')}
        />
      </React.Fragment>
    )
  }

  const onClick = (name, rowData) => {
    dialogFuncMap[`${name}`](true)
    //setSelectedProposal(rowData)
  }

  const onHide = (name) => {
    dialogFuncMap[`${name}`](false)
  }

  const filtersMap = {
    filters1: { value: filters1, callback: setFilters1 },
  }

  const renderHeader = (filtersKey) => {
    const filters = filtersMap[`${filtersKey}`].value
    const value = filters['global'] ? filters['global'].value : ''
    return (
      <span className="p-input-icon-left">
        <i className="pi pi-search" />
        <InputText
          type="search"
          value={value || ''}
          onChange={(e) => onGlobalFilterChange(e, filtersKey)}
          placeholder="Global Search"
        />
      </span>
    )
  }

  const header1 = renderHeader('filters1')

  const proposalTypeTemplate = (rowData) => {
    return (
      <React.Fragment>
        {rowData.proposal_type === 1 ? (
          <>Buy LTS</>
        ) : rowData.proposal_type === 2 ? (
          <>Sell LTS</>
        ) : (
          <> Fund project</>
        )}
      </React.Fragment>
    )
  }

  const actionBodyTemplate = (rowData) => {
    return (
      <React.Fragment>
        <div className="container mt-4">
          <div className="row">
            <div className="col-md-4">
              <Button
                label="View"
                className="btn p-button-success mr-2"
                onClick={() => onClick('displayProposalDialog', rowData)}
              />
            </div>
            <div className="col-md-4">
              <Button
                label="Vote"
                className="btn mr-2"
                style={{ backgroundColor: '#ffde00' }}
                onClick={() => onClick('displayVoteDialog', rowData)}
                disabled={rowData.vote_is_expired}
              />
            </div>
            {rowData?.vote_is_expired ? (
              <div className="col-md-4">
                <Button
                  label="Resolve"
                  className="btn mr-2"
                  style={{ backgroundColor: '#32a877' }}
                  onClick={() => onClick('displayResolveDialog', rowData)}
                />
              </div>
            ) : (
              <></>
            )}
          </div>
        </div>
        &nbsp;
      </React.Fragment>
    )
  }

  const liveBodyTemplate = (rowData) => {
    return (
      <>
        {!rowData.vote_is_expired ? (
          <Button
            label="Live"
            className="p-button-rounded p-button-success mr-2 button-live"
            disabled
            style={{ color: 'white' }}
          />
        ) : (
          <></>
        )}
      </>
    )
  }

  useEffect(() => {}, [])

  return (
    <div className="container mt-4">
      <div className="row">
        <div className="col-md-6">
          <Section>
            <div className="title-container">
              <div className="title">
                <h4>Treasury assets</h4>
              </div>
            </div>
          </Section>
        </div>
        <div className="col-md-6">
          <Section></Section>
        </div>
      </div>
      <div className="row mt-4">
        <div className="col-md-12">
          <Section>
            <div className="title-container">
              <div className="title">
                <h4>Proposals</h4>
              </div>
            </div>

            <Toolbar
              className="mb-4 mt-4"
              right={rightToolbarTemplate}
            ></Toolbar>

            <DataTable
              className="mt-4"
              value={proposals}
              responsiveLayout="scroll"
              paginator
              rows={10}
              header={header1}
              filters={filters1}
              onFilter={(e) => setFilters1(e.filters)}
              onSelectionChange={(e) => setSelectedProposal(e.value)}
              dataKey="id"
              stateStorage="session"
              stateKey="dt-state-demo-session"
              emptyMessage={'No proposals'}
            >
              <Column field="proposal_title" header="Title"></Column>
              <Column header="Type" body={proposalTypeTemplate}></Column>
              <Column field="votes_against" header="Votes against"></Column>
              <Column field="votes_for" header="Votes for"></Column>
              <Column field="amount" header="Amount"></Column>
              <Column
                field="formattedDeadline"
                header="Deadline"
                sortable
              ></Column>
              <Column header="" body={actionBodyTemplate}></Column>
              <Column
                field="vote_is_expired"
                header=""
                body={liveBodyTemplate}
              ></Column>
            </DataTable>
          </Section>
        </div>
      </div>
      <Dialog
        header="Create a proposal"
        visible={displayBasic}
        style={{ width: '50vw' }}
        onHide={() => onHide('displayBasic')}
      ></Dialog>
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

export default CouncilPage
