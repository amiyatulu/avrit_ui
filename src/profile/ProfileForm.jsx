import React, { useState } from "react"
import { Field } from "formik"

function ProfileForm(props) {
  const { errors, touched, isSubmitting} = props

  return (
    <React.Fragment>
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
        <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
          Submit Form
        </button>
      </div>
    </React.Fragment>
  )
}

export default ProfileForm
