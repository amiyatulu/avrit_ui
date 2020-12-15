import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory } from "react-router-dom"
import { NearContext } from "../context/NearContext"
import ipfs from "../commons/ipfs"
import { FocusError, SubmittingWheel } from "../commons/FocusWheel"
import Ipfsadd from '../commons/TextileIO';

function CreateProduct(props) {
  // const [count, setCount] = useState(0);
  let history = useHistory()
  let nearvar = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false);

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            headline: "",
            introduction: "",
            details: "",
            profile_type: "",
            specialization: "",
            audience: "",
          }}
          validationSchema={Yup.object().shape({
            headline: Yup.string().required("headline is required"),
            introduction: Yup.string().required("introduction is required"),
            details: Yup.string().required("details is required"),
            profile_type: Yup.string().required("profile_type is required"),
            specialization: Yup.string().required("specialization is required"),
            audience: Yup.string().required("audience is required"),
          })}
          onSubmit={async (values, actions) => {
            //   values.countvariable = count
            try {
              const profile_type = values.profile_type
              let clonedValues = {...values}
              delete clonedValues.profile_type
              console.log(clonedValues)
              const file = await ipfs.add({
                path: "product.json",
                content: JSON.stringify(clonedValues),
              })
              await nearvar.contract.create_product({ product_details_hash: file.cid.string, product_type: profile_type})

              // const content = JSON.stringify(values);
              // const filename = "product.json"
              // const data = await Ipfsadd(content, filename)
              // await nearvar.contract.create_product({ product_details_hash: data.path.cid.string })
            } catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }

            // actions.setSubmitting(false)
            // console.log(data)
            // history.push(`/thankyou`)
          }}
        >
          {({ handleSubmit, handleBlur, handleChange, errors, touched, isValid, isSubmitting, values, setFieldValue, validateForm }) => (
            <Form onSubmit={handleSubmit}>
              {errorThrow && <p>{errorThrow}</p>}
              <div className="form-group">
                <label htmlFor="headline">headline</label>
                {touched.headline && errors.headline && <p className="alert alert-danger">{errors.headline}</p>}

                <Field name="headline" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="introduction">introduction</label>
                {touched.introduction && errors.introduction && <p className="alert alert-danger">{errors.introduction}</p>}

                <Field name="introduction" component="textarea" rows="5" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="details">details</label>
                {touched.details && errors.details && <p className="alert alert-danger">{errors.details}</p>}

                <Field name="details" component="textarea" rows="5" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="profile_type">profile_type</label>
                {touched.profile_type && errors.profile_type && <p className="alert alert-danger">{errors.profile_type}</p>}

                <Field name="profile_type" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="specialization">specialization</label>
                {touched.specialization && errors.specialization && <p className="alert alert-danger">{errors.specialization}</p>}

                <Field name="specialization" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="audience">audience</label>
                {touched.audience && errors.audience && <p className="alert alert-danger">{errors.audience}</p>}

                <Field name="audience" className="form-control" />
              </div>

              <div className="text-center">
                <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
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
