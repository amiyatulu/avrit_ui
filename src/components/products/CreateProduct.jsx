import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import TagsInput from "./TagsInput"
import DropProductImage from "./DropProductImage"
import DropProductPDFs from "./DropProductPDFs"
import ReactQuill from "react-quill"
import "react-quill/dist/quill.snow.css"
import "./CreateProduct.css"

function CreateProduct(props) {
  // const [count, setCount] = useState(0);
  let modules = {
    toolbar: [
      [{ 'header': [1, 2, false] }],
      ['bold', 'italic', 'underline','strike', 'blockquote'],
      [{'list': 'ordered'}, {'list': 'bullet'}, {'indent': '-1'}, {'indent': '+1'}],
      ['link', 'image', 'video'],
      ['clean']
    ],
  }

  let formats = [
    'header',
    'bold', 'italic', 'underline', 'strike', 'blockquote',
    'list', 'bullet', 'indent',
    'link', 'image', 'video'
  ]
  let history = useHistory()
  const { pt } = useParams()
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
            details: "",
            pdfs: "",
            specialization: "",
            audience: "",
          }}
          validationSchema={Yup.object().shape({
            headline: Yup.string().required("Title is required"),
            productimage: Yup.string().required(
              "Image is required and it should be JPG or PNG"
            ),
            details: Yup.string().required("Details is required"),
            pdfs: Yup.string(),
            specialization: Yup.string().required("Specialization is required"),
            audience: Yup.string().required("Audience is required"),
          })}
          onSubmit={async (values, actions) => {
            //   values.countvariable = count
            try {
              let profile_type
              if (pt === "cm") {
                profile_type = "oa"
              }
              if (pt === "as") {
                profile_type = "ev"
              } else {
                profile_type = pt
              }
              values.profile_type_fullname = pt

              // console.log(values)
              // console.log(pt)
              const file = await ipfs({
                path: "product.json",
                content: JSON.stringify(values),
              })
              await nearvar.contract.create_product({
                product_details_hash: file.cid.string,
                product_type: profile_type,
              })

              // const content = JSON.stringify(values);
              // const filename = "product.json"
              // const data = await Ipfsadd(content, filename)
              // await nearvar.contract.create_product({ product_details_hash: data.path.cid.string })
              history.push("/myproducts")
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
            setFieldTouched,
            validateForm,
          }) => (
            <Form onSubmit={handleSubmit}>
              <br />
              {errorThrow && <p>{errorThrow}</p>}
              <div className="form-group">
                <label htmlFor="headline">Title</label>
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
                setFieldTouched={setFieldTouched}
              />
              {/* <TagsInput selectedTags={selectedTags} name={"audience"} setFieldValue={setFieldValue} tags={['Novice', 'Intermediate']}/> */}

              <div className="form-group">
                <label htmlFor="details">Details</label>
                {touched.details && errors.details && (
                  <p className="alert alert-danger">{errors.details}</p>
                )}

                <Field id="details" name="details" className="form-control">
                  {({ field }) => (
                    <ReactQuill
                      value={field.value}
                      onChange={field.onChange(field.name)}
                      modules={modules}
                      formats={formats}
                      // modules={CreateProduct.modules}
                    />
                  )}
                </Field>
              </div>
              <div className="form-group">
                <label htmlFor="PDFs">PDFs</label>
                {touched.pdfs && errors.pdfs && (
                  <p className="alert alert-danger">{errors.pdfs}</p>
                )}
                <DropProductPDFs name={"pdfs"} setFieldValue={setFieldValue} />
              </div>

              {/* <div className="form-group">
                <label htmlFor="audience">Link Product Id:</label>
                <div className="p-3 mb-2 bg-light text-dark">
                  This field is for linking the product with other products.
                  <br />
                  For example, if you are submitting solution of assignment,
                  please link the assignment product id number.
                </div>
                {touched.linkproductid && errors.linkproductid && (
                  <p className="alert alert-danger">{errors.linkproductid}</p>
                )}
                <TagsInput
                  selectedTags={selectedTags}
                  name={"linkproductid"}
                  setFieldValue={setFieldValue}
                  tags={[]}
                />
              </div> */}

              <div className="form-group">
                <label htmlFor="specialization">Specialization</label>
                {touched.specialization && errors.specialization && (
                  <p className="alert alert-danger">{errors.specialization}</p>
                )}
                <TagsInput
                  selectedTags={selectedTags}
                  name={"specialization"}
                  setFieldValue={setFieldValue}
                  tags={["Calculus", "Blockchain"]}
                />
                {/* <TagsInput name="specialization" value={values.specialization} onChange={specialization => {
                  console.log(specialization)
                  setFieldValue("specialization", specialization)
                }} /> */}
              </div>

              <div className="form-group">
                <label htmlFor="audience">Audience</label>
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
              <SubmittingWheel isSubmitting={isSubmitting} />
              <div className="text-center">
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={isSubmitting}
                >
                  Submit Form
                </button>
              </div>

              <FocusError />
            </Form>
          )}
        </Formik>
      </div>
    </React.Fragment>
  )
}





export default CreateProduct

