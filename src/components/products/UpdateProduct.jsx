import React, { useState, useContext, useEffect } from "react"
import * as Yup from "yup"
import axios from "axios"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import { NearContext } from "../../commons/context/NearContext"
import ipfs from "../../commons/ipfs"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import Ipfsadd from "../../commons/TextileIO"
import TagsInput from "./TagsInput"
import DropProductImageUpdate from "./DropProductImageUpdate"
import { IPFS_URL } from "../../config/configvar"
import DropProductPDFsUpdate from "./DropProductPDFsUpdate"
import ReactQuill from "react-quill"
import "react-quill/dist/quill.snow.css"
import "./CreateProduct.css"

function UpdateProduct(props) {
  // const [count, setCount] = useState(0);
  let modules = {
    toolbar: [
      [{ header: [1, 2, false] }],
      ["bold", "italic", "underline", "strike", "blockquote"],
      [
        { list: "ordered" },
        { list: "bullet" },
        { indent: "-1" },
        { indent: "+1" },
      ],
      ["link", "image", "video"],
      ["clean"],
    ],
  }

  let formats = [
    "header",
    "bold",
    "italic",
    "underline",
    "strike",
    "blockquote",
    "list",
    "bullet",
    "indent",
    "link",
    "image",
    "video",
  ]
  let history = useHistory()
  const { pid } = useParams()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  const [fetchProductError, setFetchProductError] = useState(false)
  const [productType, setProductType] = useState(null)
  const [ipfsData, setProductData] = useState(null)
  const selectedTags = (tags) => {
    console.log(tags)
  }

  useEffect(() => {
    async function fetchProduct() {
      console.log(pid)
      try {
        let data = await nearvar.contract.get_product_js({
          product_id: pid.toString(),
        })
        console.log(data)
        setProductType(data.product_type)
        const result = await axios(`${IPFS_URL}${data.product_details_hash}`)
        console.log(result.data)
        setProductData(result.data)
      } catch (e) {
        setFetchProductError(e)
        console.error(e)
      }
    }

    fetchProduct()
  }, [])
  if (ipfsData && productType) {
    return (
      <React.Fragment>
        <div className="container">
          <Formik
            initialValues={{
              headline: ipfsData.headline,
              productimage: ipfsData.productimage,
              details: ipfsData.details,
              pdfs: ipfsData.pdfs,
              specialization: ipfsData.specialization,
              audience: ipfsData.audience,
            }}
            validationSchema={Yup.object().shape({
              headline: Yup.string().required("Title is required"),
              productimage: Yup.string().required(
                "Image is required and it should be JPG or PNG"
              ),
              details: Yup.string().required("Details is required"),
              pdfs: Yup.string().required("Upload the PDFs"),
              specialization: Yup.string().required(
                "Specialization is required"
              ),
              audience: Yup.string().required("Audience is required"),
            })}
            onSubmit={async (values, actions) => {
              values.profile_type_fullname = ipfsData.profile_type_fullname
              // values.profile_type_fullname = productType
              try {
                const file = await ipfs({
                  path: "product.json",
                  content: JSON.stringify(values),
                })
                await nearvar.contract.update_product({
                  product_details_hash: file.cid.string,
                  product_id: pid.toString(),
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
                <DropProductImageUpdate
                  name={"productimage"}
                  setFieldValue={setFieldValue}
                  setFieldTouched={setFieldTouched}
                  oldimage={ipfsData.productimage}
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
                  <DropProductPDFsUpdate
                    name={"pdfs"}
                    setFieldValue={setFieldValue}
                    ipfsoldpaths={ipfsData.pdfs}
                  />
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
                    tags={linkproduct}
                  />
                </div> */}

                <div className="form-group">
                  <label htmlFor="specialization">Specialization</label>
                  {touched.specialization && errors.specialization && (
                    <p className="alert alert-danger">
                      {errors.specialization}
                    </p>
                  )}
                  <TagsInput
                    selectedTags={selectedTags}
                    name={"specialization"}
                    setFieldValue={setFieldValue}
                    tags={ipfsData.specialization.split(",")}
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
                    tags={ipfsData.audience.split(",")}
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
  } else if (fetchProductError) {
    return (
      <React.Fragment>
        {JSON.stringify(fetchProductError.message)}
      </React.Fragment>
    )
  } else {
    return (
      <React.Fragment>
        <div className="container">
          <div className="d-flex justify-content-center">
            <div className="spinner-grow text-warning" role="status">
              <span className="sr-only">Loading...</span>
            </div>
          </div>
        </div>
      </React.Fragment>
    )
  }
}

export default UpdateProduct
