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
  const [errorApplication, setApplicationError] = useState(false)
  const [errorJurySelectionTime, setJurySelectionTimeError] = useState(false)
  const [time, setTime] = useState(null)
  const [endcommit, setEndCommit] = useState(null)
  const [endreveal, setEndReveal] = useState(null)
  // const { rid } = useParams()
  // console.log("rid", rid)
  useEffect(() => {
    async function fetchJuryApplicationTime() {
      try {
        let phasetimeinsec = await nearvar.contract.get_jury_application_phase_time()
        // console.log(phasetimeinsec)
        let starttime = await nearvar.contract.get_jury_application_start_time_js(
          {
            review_id: parseInt(rid),
          }
        )
        // console.log(starttime.slice(0, 10))
        const endtime = moment.unix(
          parseInt(starttime.slice(0, 10)) + parseInt(phasetimeinsec)
        )
        // console.log(endtime)
        // console.log(endtime.fromNow())
        return endtime
      } catch (e) {
        console.log(e)
        setApplicationError(true)
      }
    }
    async function fetchJurySelectionTime() {
      try {
        let juryselectiontime = await nearvar.contract.get_juror_selection_time_js(
          {
            review_id: parseInt(rid),
          }
        )

        // console.log("juryselectiontime", juryselectiontime)
        return juryselectiontime
      } catch (e) {
        console.log(e)
        setJurySelectionTimeError(true)
      }
    }

    async function fetchCommitPhaseTime() {
      try {
        let commitphasetime = await nearvar.contract.get_commit_phase_time()
        return commitphasetime
      } catch (e) {
        console.log(e)
      }
    }

    async function fetchRevealPhaseTime() {
      try {
        let revealphasetime = await nearvar.contract.get_reveal_phase_time()
        // console.log("revealphasetime", revealphasetime)
        return revealphasetime
      } catch (e) {
        console.log(e)
      }
    }

    function fetchCommitPhaseEndTime(juryselectiontime, commitphasetime) {
      // console.log(
      //   "ccjuryselectiontime",
      //   moment.unix(parseInt(juryselectiontime.slice(0, 10)))
      // )
      let endcommitdata = moment.unix(
        parseInt(juryselectiontime.slice(0, 10)) + parseInt(commitphasetime)
      )
      // console.log("cccomitphase", commitphasetime)
      // console.log("endcommit", endcommitdata)
      return endcommitdata
    }

    function fetchRevealPhaseEndTime(
      juryselectiontime,
      commitphasetime,
      revealphasetime
    ) {
      let endreveal = moment.unix(
        parseInt(juryselectiontime.slice(0, 10)) +
          parseInt(commitphasetime) +
          parseInt(revealphasetime)
      )
      return endreveal
    }
    async function callfetchJuryApplicationTime() {
      const endtime = await fetchJuryApplicationTime()
      setTime(endtime)
    }

    async function setEndCommitEndReveal() {
      const juryselectiontime = await fetchJurySelectionTime()
      // console.log(juryselectiontime)
      const commitphasetime = await fetchCommitPhaseTime()
      // console.log(commitphasetime)
      const revealphasetime = await fetchRevealPhaseTime()
      const endcommit = fetchCommitPhaseEndTime(
        juryselectiontime,
        commitphasetime
      )
      // console.log("endcommit", endcommit)

      const endreveal = fetchRevealPhaseEndTime(
        juryselectiontime,
        commitphasetime,
        revealphasetime
      )
      // console.log("endreveal", endreveal)
      setEndCommit(endcommit)
      setEndReveal(endreveal)
    }

    callfetchJuryApplicationTime()
    setEndCommitEndReveal()
  }, [nearvar, rid])

  // if reveal time ends

  // else if commit time ends

  // else if application time ends
  // console.log("endreveal", endreveal)
  if (moment().isSameOrAfter(endreveal)) {
    console.log("reveal time ends")
    return <React.Fragment></React.Fragment>
  }

  if (moment().isSameOrAfter(endcommit)) {
    console.log("commit time ends")
    return (
      <React.Fragment>
        <br />
        <span>Commit end time: {endcommit.fromNow()}</span> <br />
      </React.Fragment>
    )
  }
  // console.log(time)
  if (moment().isSameOrAfter(time)) {
    return (
      <React.Fragment>
        <br />
        <span>Jury application end time: {time.fromNow()}</span> <br />
        <Link to={`/commitvote/${rid}/`} className="badge badge-secondary mr-3">
          Commit Vote
        </Link>
      </React.Fragment>
    )
  }
  return <React.Fragment></React.Fragment>
}

export default TimeConditionRender
