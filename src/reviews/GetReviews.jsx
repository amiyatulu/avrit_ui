import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import { Link, NavLink } from "react-router-dom"
import ReviewFullDetailsEvidence from "./ReviewFullDetailsEvidence"

function GetReviews(props) {
  const nearcontract = useContext(NearContext)
  const [reviewsData, setReviewsData] = useState([])
  const { pid } = props

  useEffect(() => {
    async function fetchReviews() {
      let data
      try {
        data = await nearcontract.contract.get_review_ids_by_product_id({
          start: 0,
          end: 20,
          product_id: parseInt(pid),
        })
        console.log(data)
        data.map(async (x) => {
            let hash = await nearcontract.contract.get_review({review_id: x})
            
            hash.review_id = x
            console.log(hash)
            setReviewsData((oldReviews) => [...oldReviews, hash])
        })
      } catch (e) {
        console.error(e)
      }
    }
    fetchReviews()
  }, [nearcontract])
return (<React.Fragment>{
    reviewsData.length > 0 && (<div>
            <h3>Reviews</h3>
        {reviewsData.map((data) => (
            <React.Fragment key={data.review_id}>
                <ReviewFullDetailsEvidence ipfshash={data.review_hash} rid={data.review_id} />
            </React.Fragment>
        ))
        }
    </div>)
    }</React.Fragment>)
}

export default GetReviews
