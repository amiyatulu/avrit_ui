import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import keccak256 from "keccak256"

function PCommitVote(props) {
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
            Vote: "",
          }}
          validationSchema={Yup.object().shape({
            Vote: Yup.string()
              .matches(/^[10]+/, "Vote is invalid")
              .required("Vote is required"),
          })}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
              const hex = keccak256(values.Vote).toString("hex")
              const data = await nearvar.contract.p_commit_vote({
                product_id: pid.toString(),
                vote_commit: hex,
              })
              console.log(data)
              actions.setSubmitting(false)
              // console.log(data)
              history.push({
                pathname: "/commitsubmitted",
                state: { hex: hex, vote: values.Vote },
              })
              // history.goBack()
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
                <p className="p-2 mb-2 bg-primary text-white">
                  <label htmlFor="Vote">Vote:</label>
                </p>
                <p>Vote if already not voted.</p>
                <p>
                  Vote format, first character can be 0 or 1, then a unique
                  string.
                  <br />1 =&gt; 👍 Review meets the quality check as per the
                  guidelines. <br />
                  0 =&gt; 👎 Review does not meet the quality check as per the
                  guidelines.
                  <br />
                  For example, <br />
                  0iilzmfeofopzblgycbuiahhkptp <br />
                  1psiycigusjdkfoartn <br />
                  0lbjvjgzqwigattqdqglzxxdepmwnsf <br />
                </p>
                <p className="alert alert-warning">
                  Please copy the vote and the hex string in safe place before
                  submitting. You will need it to reveal the vote.
                </p>
                {errors.Vote && (
                  <p className="alert alert-danger">{errors.Vote}</p>
                )}
                <Field name="Vote" className="form-control" />
                <br />
                {values.Vote && !errors.Vote ? (
                  <p>
                    Your hex string: <br />
                    {keccak256(values.Vote).toString("hex")}
                  </p>
                ) : (
                  <p></p>
                )}
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

export default PCommitVote
