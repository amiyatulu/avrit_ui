import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import Rating from "@material-ui/lab/Rating"
import DropProductPDFs from "../products/DropProductPDFs"

function CreateReviewEvidence(props) {
  // const [count, setCount] = useState(0);
  let history = useHistory()
  const { pid } = useParams()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  const [product_rating, setProduct_rating] = React.useState(null)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            productRating:"",
            originality: "",
            probingquestion: "",
            graphics: "",
            concrete: "",
            practice: "",
            proposition: "",
            cognitiveload: "",
            pdfs: "",
          }}
          validationSchema={Yup.object().shape({
            productRating: Yup.number()
            .typeError("Product rating is required")
            .required("Product rating is required"),
            originality: Yup.string().required("originality is required"),
            probingquestion: Yup.string().required(
              "probingquestion is required"
            ),
            graphics: Yup.string().required("graphics is required"),
            concrete: Yup.string().required("concrete is required"),
            practice: Yup.string().required("practice is required"),
            proposition: Yup.string().required("proposition is required"),
            cognitiveload: Yup.string().required("cognitiveload is required"),
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
              <div className="form-group">
              <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="ProductRating">Product Rating</label>
                </p>
              {touched.productRating && errors.productRating && <p className="alert alert-danger">{errors.productRating}</p>}{" "}
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
                  <label htmlFor="originality">Originality</label>
                </p>
                {touched.originality && errors.originality && (
                  <p className="alert alert-danger">{errors.originality}</p>
                )}

                <Field
                  name="originality"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="probingquestion">
                    Posing Probing Question
                  </label>
                </p>
                {touched.probingquestion && errors.probingquestion && (
                  <p className="alert alert-danger">{errors.probingquestion}</p>
                )}

                <Field
                  name="probingquestion"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="graphics">Pairing graphics with words</label>
                </p>
                {touched.graphics && errors.graphics && (
                  <p className="alert alert-danger">{errors.graphics}</p>
                )}

                <Field
                  name="graphics"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="concrete">Abstract to Concrete</label>
                </p>
                {touched.concrete && errors.concrete && (
                  <p className="alert alert-danger">{errors.concrete}</p>
                )}

                <Field
                  name="concrete"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="practice">
                    Retrieval Practice and Component Practice
                  </label>
                </p>
                {touched.practice && errors.practice && (
                  <p className="alert alert-danger">{errors.practice}</p>
                )}

                <Field
                  name="practice"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="proposition">Logical Statements</label>
                </p>
                {touched.proposition && errors.proposition && (
                  <p className="alert alert-danger">{errors.proposition}</p>
                )}

                <Field
                  name="proposition"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="cognitiveload">Cognitive Load</label>
                </p>
                {touched.cognitiveload && errors.cognitiveload && (
                  <p className="alert alert-danger">{errors.cognitiveload}</p>
                )}

                <Field
                  name="cognitiveload"
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
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={isSubmitting}
                >
                  Submit Form
                </button>
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

export default CreateReviewEvidence
