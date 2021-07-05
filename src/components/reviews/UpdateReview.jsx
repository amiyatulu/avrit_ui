import React, { useState, useContext, useEffect } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import Rating from "@material-ui/lab/Rating"
import DropProductPDFsUpdate from "../products/DropProductPDFsUpdate"
import { IPFS_URL } from "../../config/configvar"
import axios from "axios"

function UpdateReview(props) {
  // const [count, setCount] = useState(0);
  let history = useHistory()
  const { rid } = useParams()
  const [pid, setPid] = useState(false)
  const [ipfsData, setIpfsData] = useState(false)
  let { nearvar, login } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  const [fetchReviewError, setFetchReviewError] = useState(false)
  const [product_rating, setProduct_rating] = React.useState(null)

  useEffect(() => {
    async function fetchReview() {
      try {
        let hash = await nearvar.contract.get_review_js({
          review_id: rid.toString(),
        })
        setPid(hash.product_id)
        console.log(hash.product_id)
        setProduct_rating(hash.rating)
        console.log(hash.rating, "rating")
        const result = await axios(`${IPFS_URL}${hash.review_hash}`)
        setIpfsData(result.data)
        console.log(result.data)
      } catch (e) {
        console.error(e)
        setFetchReviewError(e.message)
      }
    }

    fetchReview()
  }, [])
  if (ipfsData) {
    return (
      <React.Fragment>
        <div className="container">
          <Formik
            initialValues={{
              productRating: product_rating,
              text: ipfsData.text,
              pdfs: ipfsData.pdfs,
            }}
            validationSchema={Yup.object().shape({
              productRating: Yup.number()
                .typeError("Product rating is required")
                .required("Product rating is required"),
              text: Yup.string().required("Review text is required"),
              pdfs: Yup.string(),
            })}
            onSubmit={async (values, actions) => {
              values.productRating = product_rating
              //   const data = await ...
              // console.log(values)
              try {
                const file = await ipfs({
                  path: "review.json",
                  content: JSON.stringify(values),
                })
                console.log(file)
                console.log(pid)
                await nearvar.contract.update_review({
                  review_id: rid.toString(),
                  review_hash: file.cid.string,
                  rating: parseInt(product_rating),
                })
                actions.setSubmitting(false)
                history.push(`/product/${pid}`)
              } catch (e) {
                console.error(e)
                setErrorThrow(e.message)
              }

              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
            }}
          >
            {({
              handleSubmit,
              handleBlur,
              handleChange,
              errors,
              touched,
              isValid,
              isSubmitting,
              values,
              setFieldValue,
              validateForm,
            }) => (
              <Form onSubmit={handleSubmit}>
                {errorThrow && <p>{errorThrow}</p>}
                {!login && (
                  <p className="alert alert-warning">Log In to submit form</p>
                )}
                <div className="form-group">
                  <p className="p-2 mb-2 bg-primary text-white">
                    <label htmlFor="ProductRating">Product Rating</label>
                  </p>
                  {touched.productRating && errors.productRating && (
                    <p className="alert alert-danger">{errors.productRating}</p>
                  )}
                  <div className="text-center">
                    <br />
                    <Rating
                      name="productRating"
                      value={product_rating}
                      onChange={(event, newValue) => {
                        setProduct_rating(newValue)
                        console.log(newValue)
                        setFieldValue("productRating", newValue)
                      }}
                    />
                  </div>
                  <p className="p-2 mb-2 bg-primary text-white">
                    <label htmlFor="text">Review Text</label>
                  </p>
                  {touched.text && errors.text && (
                    <p className="alert alert-danger">{errors.text}</p>
                  )}
                  <Field
                    name="text"
                    component="textarea"
                    rows="5"
                    className="form-control"
                  />
                </div>

                <div className="form-group">
                  <label htmlFor="PDFs">PDFs</label>
                  {touched.pdfs && errors.pdfs && (
                    <p className="alert alert-danger">{errors.pdfs}</p>
                  )}
                  <DropProductPDFsUpdate
                    name={"pdfs"}
                    setFieldValue={setFieldValue}
                    ipfsoldpaths={ipfsData.pdfs}
                  />
                </div>

                <div className="text-center">
                  {login ? (
                    <button
                      type="submit"
                      className="btn btn-primary"
                      disabled={isSubmitting}
                    >
                      Submit Form
                    </button>
                  ) : (
                    <p className="alert alert-info">Log In to submit form</p>
                  )}
                </div>
                <SubmittingWheel isSubmitting={isSubmitting} />
                <FocusError />
              </Form>
            )}
          </Formik>
        </div>
      </React.Fragment>
    )
  } else if (fetchReviewError) {
    return <React.Fragment>{JSON.stringify(fetchReviewError)}</React.Fragment>
  } else {
    return (
      <React.Fragment>
        <div className="container">
          <div className="d-flex justify-content-center">
            <div className="spinner-grow text-warning" role="status">
              <span className="sr-only">Loading...</span>
            </div>
          </div>
        </div>
      </React.Fragment>
    )
  }
}

export default UpdateReview
