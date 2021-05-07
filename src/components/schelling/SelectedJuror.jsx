import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { Link } from "react-router-dom"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}

function SelectedJuror(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { rid } = props
  const [fetchError, setFetchError] = useState(false)
  const [juriesSelected, setJuriesSelected] = useState(false)
  useEffect(() => {
    async function fetchjurycount() {
      try {
        const jurieselected = await nearvar.contract.get_selected_juror_count({
          review_id: rid.toString(),
        })
        console.log("canVoteValue", jurieselected)
        setJuriesSelected(jurieselected)
      } catch (e) {
        console.error(e.message)
        setFetchError(e.message)
      }
    }
    fetchjurycount()
  }, [nearvar, userId, rid])
  return (
    <React.Fragment>
      <p>Number of Juror Selected {juriesSelected}</p>
     
      {userId && <Error fetchError={fetchError} />}
    </React.Fragment>
  )
}

export default SelectedJuror
