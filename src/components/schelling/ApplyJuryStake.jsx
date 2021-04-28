import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"

function ApplyJuryStake(props) {
  // const [count, setCount] = useState(0);
  const { rid } = useParams()
  let history = useHistory()
  let { nearvar, reloadBalance} = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            stake: "",
          }}
          validationSchema={Yup.object().shape({
            stake: Yup.number().required("stake is required"),
          })}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
              await nearvar.contract.apply_jurors({
                review_id: rid.toString(),
                stake: values.stake.toString(),
              })
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              reloadBalance()
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
                <label htmlFor="stake">stake</label>
                {touched.stake && errors.stake && (
                  <p className="alert alert-danger">{errors.stake}</p>
                )}

                <Field name="stake" className="form-control" />
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

export default ApplyJuryStake
