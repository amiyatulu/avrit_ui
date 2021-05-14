import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import moment from "moment"
import CommitVoteLink from "./CommitVoteLink"
import SelectedJuror from "./SelectedJuror"
import UnstakeVoteLink from "./UnstakeVoteLInk"
import JurorRevealLink from "./JurorRevealLink"
import GetTrueCount from "./GetTrueCount"
import GetFalseCount from "./GetFalseCount"
import JurorGetIncentivesLink from "./JurorGetIncentivesLink"

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
  const [jurySelectionTime, setJurySelectionTime] = useState(false)
  const [time, setTime] = useState(null)
  const [endcommit, setEndCommit] = useState(null)
  const [endreveal, setEndReveal] = useState(null)
  // const { rid } = useParams()
  // console.log("rid", rid)
  useEffect(() => {
    async function fetchJuryApplicationTime() {
      let endtime = false
      try {
        let phasetimeinsec =
          await nearvar.contract.get_jury_application_phase_time()
        console.log("juryapplicatiophasetimeinsec", phasetimeinsec)
        let starttime =
          await nearvar.contract.get_jury_application_start_time_js({
            review_id: rid.toString(),
          })
        const endtime = moment.unix(
          parseInt(starttime.slice(0, 10)) + parseInt(phasetimeinsec)
        )

        return endtime
        // console.log(endtime)
        // console.log(endtime.fromNow())
      } catch (e) {
        console.log(e)
        setApplicationError(true)
      }
      return endtime
    }
    async function fetchJurySelectionTime() {
      try {
        let juryselectiontime =
          await nearvar.contract.get_juror_selection_time_js({
            review_id: rid.toString(),
          })

        // console.log("juryselectiontime", juryselectiontime)
        return juryselectiontime
      } catch (e) {
        console.log(e)
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
      let endcommitdata = false
      try {
        endcommitdata = moment.unix(
          parseInt(juryselectiontime.slice(0, 10)) + parseInt(commitphasetime)
        )
      } catch (e) {
        console.log(e)
      }

      // console.log("cccomitphase", commitphasetime)
      // console.log("endcommit", endcommitdata)
      return endcommitdata
    }

    function fetchRevealPhaseEndTime(
      juryselectiontime,
      commitphasetime,
      revealphasetime
    ) {
      let endreveal = false
      try {
        endreveal = moment.unix(
          parseInt(juryselectiontime.slice(0, 10)) +
            parseInt(commitphasetime) +
            parseInt(revealphasetime)
        )
      } catch (e) {
        console.log(e)
      }
      return endreveal
    }
    async function callfetchJuryApplicationTime() {
      const endtime = await fetchJuryApplicationTime()
      console.log(endtime, "endtime")
      setTime(endtime)
    }

    async function setEndCommitEndReveal() {
      const juryselectiontime = await fetchJurySelectionTime()
      setJurySelectionTime(juryselectiontime)
      console.log(juryselectiontime, "juryselectiontime")
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
  console.log("endreveal", endreveal)
  if (moment().isSameOrAfter(endreveal)) {
    console.log("reveal time ends")
    return (
      <React.Fragment>
        <br />
        <span className="badge badge-info">
          Reveal Time Ended : {endreveal && endreveal.fromNow()}
        </span>
        <br />
        <span className="badge badge-info">You can draw your incentives</span>
        <br />
        <GetTrueCount rid={rid} />
        <GetFalseCount rid={rid} />
        <JurorGetIncentivesLink rid={rid} />
      </React.Fragment>
    )
  }

  if (moment().isSameOrAfter(endcommit)) {
    console.log("commit time ends")
    return (
      <React.Fragment>
        <br />
        {/* <span className="badge badge-info">Commit end time: {endcommit.fromNow()}</span> <br /> */}
        <span className="badge badge-info">
          End reveal time : {endreveal && endreveal.fromNow()}
        </span>{" "}
        <br />
        <JurorRevealLink rid={rid} /> <br />
        <GetTrueCount rid={rid} />
        <GetFalseCount rid={rid} />
      </React.Fragment>
    )
  }

  // console.log(jurySelectionTime, "jury selection time");
  // console.log(time, "time")
  // console.log(endcommit, "commitendtime")

  let juryselectiontime_slice
  if (jurySelectionTime) {
    juryselectiontime_slice = moment.unix(jurySelectionTime.slice(0, 10))
  }

  if (
    moment().isSameOrAfter(time) &&
    moment().isSameOrAfter(juryselectiontime_slice) &&
    jurySelectionTime !== undefined
  ) {
    return (
      <React.Fragment>
        <br />
        <span className="badge badge-info">
          Commit end time: {endcommit && endcommit.fromNow()}
        </span>{" "}
        <br />
        {/* To do: If already commited don't render commit vote */}
        <SelectedJuror rid={rid} />
        <UnstakeVoteLink rid={rid} />
        <CommitVoteLink rid={rid} />
      </React.Fragment>
    )
  }

  if (jurySelectionTime === undefined && moment().isSameOrAfter(time)) {
    return (
      <React.Fragment>
        <SelectedJuror rid={rid} />
        <Link to={`/drawjurors/${rid}`} className="badge badge-secondary mr-3">
          Draw Juror
        </Link>
      </React.Fragment>
    )
  }

  if (moment().isSameOrBefore(time)) {
    return (
      <React.Fragment>
        <br />
        <span className="badge badge-info">
          Jury application end time: {time.fromNow()}
        </span>{" "}
        <br />
      </React.Fragment>
    )
  }

  return <React.Fragment></React.Fragment>
}

export default TimeConditionRender
