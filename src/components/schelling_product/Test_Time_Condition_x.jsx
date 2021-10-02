import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import moment from "moment"

function Loading(props) {
  const { error } = props
  if (error) {
    return <React.Fragment></React.Fragment>
  }
  return (
    <p className="container">
      Loading
      <span role="img" aria-label="loading">
        ⌛
      </span>
    </p>
  )
}

// Functions available:
// get_jury_application_start_time_js + get_jury_application_phase_time => Jury application end time
// get_juror_selection_time_js + get_commit_phase_time => End of commit phase
// get_juror_selection_time_js + get_commit_phase_time + get_reveal_phase_time => End of reveal time

function TimeConditionRender(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { rid } = props

  useEffect(() => {
    async function fetchTimes() {
      let jury_application_start_time = await nearvar.contract.get_jury_application_start_time_js(
        {
          review_id: rid.toString(),
        }
      )
      console.log("jury_application_start_time", jury_application_start_time)
    }
    fetchTimes()
  })

  return <React.Fragment></React.Fragment>
}

export default TimeConditionRender
