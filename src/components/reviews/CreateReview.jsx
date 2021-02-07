import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import Rating from "@material-ui/lab/Rating"
import DropProductPDFs from "../products/DropProductPDFs"

function CreateReview(props) {
  // const [count, setCount] = useState(0);
  let history = useHistory()
  const { pid } = useParams()
  let { nearvar, login } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  const [product_rating, setProduct_rating] = React.useState(null)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            productRating: "",
            text: "➼ Originality \n➼ Probing Questions \n➼ Graphics",
            pdfs: "",
          }}
          validationSchema={Yup.object().shape({
            productRating: Yup.number()
              .typeError("Product rating is required")
              .required("Product rating is required"),
            text: Yup.string().required("text field is required"),
            pdfs: Yup.string().required("Adding PDFs is required"),
          })}
          onSubmit={async (values, actions) => {
            values.productRating = product_rating
            //   const data = await ...
            // console.log(values)
            try {
              const file = await ipfs.add({
                path: "review.json",
                content: JSON.stringify(values),
              })
              console.log(file)
              console.log(pid)
              await nearvar.contract.create_review({
                product_id: parseInt(pid),
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
              {!login && <p className="alert alert-warning">Log In to submit form</p>}
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
                <DropProductPDFs name={"pdfs"} setFieldValue={setFieldValue} />
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
}

export default CreateReview
