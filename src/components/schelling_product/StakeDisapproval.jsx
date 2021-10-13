import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import { BigNumber } from "bignumber.js";


function StakeDisapproval(props) {
  let history = useHistory()
  let { nearvar, reloadBalance } = useContext(NearContext)
  const { id } = useParams()
  const [errorThrow, setErrorThrow] = useState(false)
  let pw = BigNumber(10).pow(18)

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
          //   values.countvariable = count
          //   const data = await ...
          try {
            let attotokens = BigNumber(values.stake).times(pw)
            await nearvar.contract.p_add_product_disapproval_bounty({
              bounty:  attotokens.toFixed(),
              product_id: id.toString(),
            })
            actions.setSubmitting(false)
            reloadBalance()
            history.goBack()
          } catch (e) {
            console.error(e)
            setErrorThrow(e.message)
          }

          // console.log(data)
          //
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

export default StakeDisapproval