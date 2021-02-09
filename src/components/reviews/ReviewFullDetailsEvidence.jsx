import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { IPFS_URL } from "../../config/configvar"
import { Link } from "react-router-dom"
import GetReviewStake from "../stakes/GetReviewStake"
import TimeConditionRender from "../schelling/TimeConditionRender"
import Rating from "@material-ui/lab/Rating"
import longwords from "../products/LongWords.module.css"
import Linkify from 'react-linkify'


function ReviewFullDetailsEvidence(props) {
  let { ipfshash, rid, rating} = props
  const { nearvar, userId } = useContext(NearContext)
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
            <h5>Ratings</h5>
            <p><Rating name="productrating" value={rating} /></p>
            <h5>Review</h5>
            <p className={`${longwords.linebreaks} ${longwords.wraplongworld}`}><Linkify>{reviewData.text}</Linkify></p>
            <Link
              to={`/reviewstake/${rid}`}
              className="badge badge-secondary mr-3"
            >
              Add or Update Stake
            </Link>
            <GetReviewStake rid={rid} />
            <TimeConditionRender rid={rid} />
            
          </div>
        </div>
      )}
    </React.Fragment>
  )
}

export default ReviewFullDetailsEvidence
