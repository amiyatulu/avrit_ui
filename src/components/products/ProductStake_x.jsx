import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import { BigNumber } from "bignumber.js";

function ProductStake(props) {
  // const [count, setCount] = useState(0);
  let history = useHistory()
  let { nearvar } = useContext(NearContext)
  let pw = BigNumber(10).pow(18)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{
            stake: "",
          }}
          validationSchema={Yup.object().shape({
            stake: Yup.string().required("stake is required"),
          })}
          onSubmit={async (values, actions) => {
            //   values.countvariable = count
            //   const data = await ...
            try {
              let attotokens = BigNumber(values.stake).times(pw)
              await nearvar.contract.add_product_bounty({
                bounty: attotokens.toFixed(),
                product_id: 1,
              })
              actions.setSubmitting(false)
            } catch (e) {
              console.error(e)
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

export default ProductStake
