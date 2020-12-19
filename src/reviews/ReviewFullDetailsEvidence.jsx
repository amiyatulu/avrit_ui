import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { IPFS_URL } from "../config/configvar"

function ReviewFullDetailsEvidence(props) {
  let { ipfshash } = props
  const nearcontract = useContext(NearContext)
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
  }, [nearcontract])
  return (
    <React.Fragment>
      {reviewData && (
        <div>
          <div className="jumbotron">
            <h5>Originality</h5>
            <p>{reviewData.originality}</p>
            <h5>Posing Probing Question</h5>
            <p>{reviewData.probingquestion}</p>
          </div>
        </div>
      )}
    </React.Fragment>
  )
}

export default ReviewFullDetailsEvidence
