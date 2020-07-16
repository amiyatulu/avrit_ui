import React, { useState } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { FocusError, SubmittingWheel } from "../commons/FocusWheel"

function CreateProfile() {
  const [count, setCount] = useState(0)

  return (
    <React.Fragment>
      <div className="container">
      <Formik
        initialValues={{
          headline: "",
          introduction: "",
          details: "",
          youAre: "",
          skills: "",
        }}
        validationSchema={Yup.object().shape({
          headline: Yup.string().required("Headline is required"),
          introduction: Yup.string().required("Introduction is required"),
          details: Yup.string().required("Details is required"),
          youAre: Yup.string().required("You are is required"),
          skills: Yup.string().required("Skills is required"),
        })}
        onSubmit={async (values, actions) => {
          //values.countvariable = count
          console.log(values)
          actions.setSubmitting(false)
          // console.log(data)
        }}
      >
        {({ handleSubmit, handleBlur, handleChange, errors, touched, isValid, isSubmitting, values, setFieldValue, validateForm }) => (
          <Form onSubmit={handleSubmit}>

            <div className="form-group">
              <label htmlFor="headline">Headline</label>
              {touched.headline && errors.headline && <p className="alert alert-danger">{errors.headline}</p>}

              <Field name="headline" className="form-control" />
            </div>

            <div className="form-group">
              <label htmlFor="introduction">Introduction</label>
              {touched.introduction && errors.introduction && <p className="alert alert-danger">{errors.introduction}</p>}

              <Field name="introduction" component="textarea" rows="5" className="form-control" />
            </div>

            <div className="form-group">
              <label htmlFor="details">Details</label>
              {touched.details && errors.details && <p className="alert alert-danger">{errors.details}</p>}

              <Field name="details" component="textarea" rows="5" className="form-control" />
            </div>

            <div className="form-group">
              <label htmlFor="youAre">You are a</label>
              {touched.youAre && errors.youAre && <p className="alert alert-danger">{errors.youAre}</p>}

              <Field name="youAre" className="form-control" />
            </div>

            <div className="form-group">
              <label htmlFor="skills">Skills</label>
              {touched.skills && errors.skills && <p className="alert alert-danger">{errors.skills}</p>}

              <Field name="skills" component="textarea" rows="5" className="form-control" />
            </div>

            <div className="text-center">
              <button type="submit" className="btn btn-primary">
                Submit Form
              </button>
            </div>
            <SubmittingWheel isValid={isValid} isSubmitting={isSubmitting} />
            <FocusError />
          </Form>
        )}
      </Formik>
      </div>
    </React.Fragment>
  )
}

export default CreateProfile
