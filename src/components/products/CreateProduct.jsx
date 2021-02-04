import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import Ipfsadd from "../../commons/TextileIO"
import TagsInput from "./TagsInput"
import DropProductImage from "./DropProductImage"
import DropProductPDFs from "./DropProductPDFs"

function CreateProduct(props) {
  // const [count, setCount] = useState(0);
  let history = useHistory()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  const selectedTags = (tags) => {
    console.log(tags)
  }

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            headline: "",
            productimage: "",
            introduction: "",
            details: "",
            pdfs: "",
            profile_type: "",
            specialization: "",
            audience: "",
          }}
          validationSchema={Yup.object().shape({
            headline: Yup.string().required("headline is required"),
            productimage: Yup.string().required("Image is required"),
            introduction: Yup.string().required("introduction is required"),
            details: Yup.string().required("Details is required"),
            pdfs: Yup.string().required("Adding PDFs is required"),
            profile_type: Yup.string().required("profile_type is required"),
            specialization: Yup.string().required("specialization is required"),
            audience: Yup.string().required("audience is required"),
          })}
          onSubmit={async (values, actions) => {
            //   values.countvariable = count
            try {
              const profile_type = values.profile_type
              let clonedValues = { ...values }
              delete clonedValues.profile_type
              console.log(clonedValues)
              // const file = await ipfs.add({
              //   path: "product.json",
              //   content: JSON.stringify(clonedValues),
              // })
              // await nearvar.contract.create_product({
              //   product_details_hash: file.cid.string,
              //   product_type: profile_type,
              // })

              // // const content = JSON.stringify(values);
              // // const filename = "product.json"
              // // const data = await Ipfsadd(content, filename)
              // // await nearvar.contract.create_product({ product_details_hash: data.path.cid.string })
              // history.push("/myproducts")
            } catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }

            // actions.setSubmitting(false)
            // console.log(data)
            // history.push(`/thankyou`)
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
            setTouched,
            validateForm,
          }) => (
            <Form onSubmit={handleSubmit}>
              {errorThrow && <p>{errorThrow}</p>}
              <div className="form-group">
                <label htmlFor="headline">headline</label>
                {touched.headline && errors.headline && (
                  <p className="alert alert-danger">{errors.headline}</p>
                )}

                <Field name="headline" className="form-control" />
              </div>

              <label htmlFor="audience">Product Image</label>
              {errors.productimage && touched.productimage && (
                <p className="alert alert-danger">{errors.productimage}</p>
              )}
              <DropProductImage
                name={"productimage"}
                setFieldValue={setFieldValue}
                setTouched={setTouched}
              />
              {/* <TagsInput selectedTags={selectedTags} name={"audience"} setFieldValue={setFieldValue} tags={['Novice', 'Intermediate']}/> */}

              <div className="form-group">
                <label htmlFor="introduction">introduction</label>
                {touched.introduction && errors.introduction && (
                  <p className="alert alert-danger">{errors.introduction}</p>
                )}

                <Field
                  name="introduction"
                  component="textarea"
                  rows="5"
                  className="form-control"
                />
              </div>

              <div className="form-group">
                <label htmlFor="details">details</label>
                {touched.details && errors.details && (
                  <p className="alert alert-danger">{errors.details}</p>
                )}

                <Field
                  name="details"
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
              <div className="form-group">
                <label htmlFor="profile_type">profile_type</label>
                {touched.profile_type && errors.profile_type && (
                  <p className="alert alert-danger">{errors.profile_type}</p>
                )}
                <div className="form-check">
                  <label>
                    <Field
                      type="radio"
                      name="profile_type"
                      className="form-check-input"
                      value="ev"
                    />
                    Evidence of Learning
                  </label>
                </div>
                <div className="form-check">
                  <label>
                    <Field
                      type="radio"
                      name="profile_type"
                      className="form-check-input"
                      value="oa"
                    />
                    Open Access
                  </label>
                </div>
                <div className="form-check">
                  <label>
                    <Field
                      type="radio"
                      name="profile_type"
                      className="form-check-input"
                      value="rm"
                    />
                    Room
                  </label>
                </div>
                <div className="form-check">
                  <label>
                    <Field
                      type="radio"
                      name="profile_type"
                      className="form-check-input"
                      value="cm"
                    />
                    Curriculum
                  </label>
                </div>
                <div className="form-check">
                  <label>
                    <Field
                      type="radio"
                      name="profile_type"
                      className="form-check-input"
                      value="oh"
                    />
                    Others
                  </label>
                </div>
              </div>

              <div className="form-group">
                <label htmlFor="specialization">specialization</label>
                {touched.specialization && errors.specialization && (
                  <p className="alert alert-danger">{errors.specialization}</p>
                )}
                <TagsInput
                  selectedTags={selectedTags}
                  name={"specialization"}
                  setFieldValue={setFieldValue}
                  tags={["All", "Calculus", "Blockchain"]}
                />
                {/* <TagsInput name="specialization" value={values.specialization} onChange={specialization => {
                  console.log(specialization)
                  setFieldValue("specialization", specialization)
                }} /> */}
              </div>

              <div className="form-group">
                <label htmlFor="audience">audience</label>
                {touched.audience && errors.audience && (
                  <p className="alert alert-danger">{errors.audience}</p>
                )}
                <TagsInput
                  selectedTags={selectedTags}
                  name={"audience"}
                  setFieldValue={setFieldValue}
                  tags={["Novice", "Intermediate"]}
                />
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

export default CreateProduct
