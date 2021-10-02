import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"

function PRevealVote(props) {
  // const [count, setCount] = useState(0);
  const { pid } = useParams()
  let history = useHistory()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            vote: "",
            vote_commit: "",
          }}
          validationSchema={Yup.object().shape({
            vote: Yup.string().required("vote is required"),
            vote_commit: Yup.string().required("vote_commit is required"),
          })}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
              await nearvar.contract.p_reveal_vote({
                product_id: pid.toString(),
                vote: values.vote.toString(),
                vote_commit: values.vote_commit.toString(),
              })
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              history.goBack()
            } catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }
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
                <label htmlFor="vote">vote</label>
                {touched.vote && errors.vote && (
                  <p className="alert alert-danger">{errors.vote}</p>
                )}

                <Field name="vote" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="vote_commit">vote_commit</label>
                {touched.vote_commit && errors.vote_commit && (
                  <p className="alert alert-danger">{errors.vote_commit}</p>
                )}

                <Field name="vote_commit" className="form-control" />
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

export default PRevealVote
