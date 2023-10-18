<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST PHAROS</name>
   <tag></tag>
   <elementGuidId>943c799c-9f57-4dde-9035-f1a38c275c3d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;idProducto\&quot;:\&quot;1\&quot;,\n  \&quot;codAgente\&quot;:13158,\n  \&quot;codPuntoVenta\&quot;:38341,\n  \&quot;tipoNegocio\&quot;:2,\n  \&quot;metadata\&quot;:\n      [{\n        \&quot;primerApellido\&quot;:\&quot;Torres\&quot;,\n        \&quot;segundoApellido\&quot;:\&quot;Galian\&quot;,\n        \&quot;nombres\&quot;:\&quot;Lina\&quot;,\n        \&quot;fechaNacimiento\&quot;:\&quot;1990-04-30\&quot;,\n        \&quot;sexo\&quot;:\&quot;F\&quot;,\n        \&quot;tipoDoc\&quot;:1,\n        \&quot;nroDoc\&quot;:\&quot;52150908\&quot;,\n        \&quot;fechaExpedicionDoc\&quot;:\&quot;2010-05-20\&quot;,\n        \&quot;direccion\&quot;:\&quot;Calle 152b No. 58c-49 Torre 2 Apto 803\&quot;,\n        \&quot;telefono\&quot;:\&quot;3134703226\&quot;,\n        \&quot;email\&quot;:\&quot;linaatorresg8@gmail.com\&quot;,\n        \&quot;ciudadDane\&quot;:11001,\n        \&quot;fechaInicio\&quot;:\&quot;2022-01-04\&quot;,\n        \&quot;fechaFinal\&quot;:\&quot;2022-02-04\&quot;,\n        \&quot;codPlan\&quot;:\&quot;PSTD\&quot;,\n        \&quot;nroPoliza\&quot;:955441070,\n        \&quot;nroAnexo\&quot;:1,\n        \&quot;peligroso\&quot;:0,\n        \&quot;nombreMascota\&quot;:\&quot;Mila\&quot;,\n        \&quot;raza\&quot;:\&quot;Border Collie\&quot;,\n        \&quot;idRaza\&quot;: \&quot;35\&quot;,\n        \&quot;color\&quot;: \&quot;marron\&quot;,\n        \&quot;edad\&quot;:6,\n        \&quot;unidadEdad\&quot;:2,\n        \&quot;chip\&quot;:0,\n        \&quot;tipoMascota\&quot;:1,\n        \&quot;generoMascota\&quot;:2,\n        \&quot;fechaExpedicion\&quot;:\&quot;2022-01-03\&quot;,\n        \&quot;codAgente\&quot;:13158,\n        \&quot;codSucursal\&quot;:14,\n        \&quot;codPuntoVenta\&quot;:38341,\n        \&quot;codTipoPago\&quot;:\&quot;TC\&quot;,\n        \&quot;valorPagado\&quot;:20000 ,\n        \&quot;nombreBanco\&quot;:\&quot;VISA\&quot;,\n        \&quot;nroAutorizacion\&quot;:\&quot;060844\&quot;,\n        \&quot;nroTarjeta\&quot;:\&quot;5801\&quot;,\n        \&quot;fechaPago\&quot;: \&quot;2022-01-03 06:13:03\&quot;,\n        \&quot;franquicia\&quot;:\&quot;VISA\&quot;\n    }\n]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_id</name>
      <type>Main</type>
      <value>${client_id}</value>
      <webElementGuid>e19ed7b4-05bc-463b-b793-91fd0f27349b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_secret</name>
      <type>Main</type>
      <value>${client_secret}</value>
      <webElementGuid>ab26df38-dadf-4e64-85c8-c7fac8bca217</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>${content_type}</value>
      <webElementGuid>906fedff-7fac-49f1-8a51-577decf6b1b8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic c2VndXJvc19tdW5kaWFsOnBlbHVkb3M=</value>
      <webElementGuid>5fffaf6e-8b4e-4f23-bf5c-b72bf8acaaf2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>resttoken</name>
      <type>Main</type>
      <value>${token}</value>
      <webElementGuid>73565065-86c9-4696-bb76-9698cc575969</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sm-pharos-sys-api.us-e2.cloudhub.io/api/v1/insurances/code</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>38e26905-1b53-422b-b5cd-274f4940c92c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_id_ph</defaultValue>
      <description></description>
      <id>f6bca4ce-3151-471b-8d90-06a4f532b70a</id>
      <masked>false</masked>
      <name>client_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_secret_ph</defaultValue>
      <description></description>
      <id>12c7c82f-cf5a-41c2-bbaa-1dce7e136d22</id>
      <masked>false</masked>
      <name>client_secret</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Content_typePH</defaultValue>
      <description></description>
      <id>8d7657f0-df87-4619-8813-fed6c13617d0</id>
      <masked>false</masked>
      <name>content_type</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
