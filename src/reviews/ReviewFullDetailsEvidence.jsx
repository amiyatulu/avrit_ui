import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { IPFS_URL } from "../config/configvar"
import { Link } from "react-router-dom"
import GetReviewStake from "../stakes/GetReviewStake"
import GetJuryStake from "../schelling/GetJuryStake"

function ReviewFullDetailsEvidence(props) {
  let { ipfshash, rid } = props
  const { nearvar } = useContext(NearContext)
  const [reviewData, setReviewData] = useState(null)

  useEffect(() => {
    async function fetchReview() {
      try {
        const result = await axios(`${IPFS_URL}${ipfshash}`)
        setReviewData(result.data)
      } catch (e) {
        console.error(e)
      }
    }
    fetchReview()
  }, [nearvar])
  return (
    <React.Fragment>
      {reviewData && (
        <div>
          <div className="jumbotron">
            <h5>Originality</h5>
            <p>{reviewData.originality}</p>
            <h5>Posing Probing Question</h5>
            <p>{reviewData.probingquestion}</p>
            <Link
              to={`/reviewstake/${rid}`}
              className="badge badge-secondary mr-3"
            >
              Add or Update Stake
            </Link>
            <GetReviewStake rid={rid} />
            <GetJuryStake rid={rid} />
          </div>
        </div>
      )}
    </React.Fragment>
  )
}

export default ReviewFullDetailsEvidence
