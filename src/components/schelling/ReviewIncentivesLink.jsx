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
   
   
function ReviewIncentivesLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { rid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canDrawIncentives, setCanDrawIncentives] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canDrawIncentivesvalue = await nearvar.contract.if_review_get_incentives_bool({
              review_id: rid.toString()
            })
            console.log("canDrawIncentivesValue", canDrawIncentivesvalue)
            setCanDrawIncentives(canDrawIncentivesvalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchjurycount()
      }, [nearvar, rid])
      return (
          <React.Fragment>
            { canDrawIncentives && <span className="badge badge-info">Reviewer can draw incentives</span>
        }
        <br />
            { canDrawIncentives && <Link to={`/drawreviewerincentives/${rid}/`} className="badge badge-secondary mr-3">
          Draw Incentives for Reviewer
        </Link>}

           {/* <p> Can You Commit Vote {JSON.stringify(canDrawIncentives)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default ReviewIncentivesLink