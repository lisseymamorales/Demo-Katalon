<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-user</name>
   <tag></tag>
   <elementGuidId>444d5f8d-4867-4098-a77e-d25a0ca233c9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_id</name>
      <type>Main</type>
      <value>b007a7c2b50e44818ac25440cf2b41f1</value>
      <webElementGuid>e0b04a9c-3b7f-4a24-94e7-5ddb06f97090</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_secret</name>
      <type>Main</type>
      <value>10Fdd81570A849289E635Da038Ea990E</value>
      <webElementGuid>801c604d-fbca-454d-a0e7-cef7feb1177d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dG9yZWkxMjM6dG9yZWkxMjM=</value>
      <webElementGuid>0f198409-20e6-4f2b-930d-23c31be6b16e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${torei-db-sys-name}/${path}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>5d22706a-9620-42f6-bfdb-ba7e7eea8408</id>
      <name>Schema GET 200</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/Get User 200.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.client_id</defaultValue>
      <description></description>
      <id>993eec04-d72c-412f-91d1-16a03b69c0fa</id>
      <masked>false</masked>
      <name>client_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_secret</defaultValue>
      <description></description>
      <id>119da7b6-254d-4c2c-aea6-9ffb20019d17</id>
      <masked>false</masked>
      <name>client_secret</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.toreidbsysname</defaultValue>
      <description></description>
      <id>c3b245c6-7b79-49ce-9171-5c1be25185bc</id>
      <masked>false</masked>
      <name>torei-db-sys-name</name>
   </variables>
   <variables>
      <defaultValue>&quot;${GlobalVariable.username}:${GlobalVariable.password}&quot;.bytes.encodeBase64().toString()</defaultValue>
      <description></description>
      <id>24e0811f-992d-4b09-945d-890ce7b1d09d</id>
      <masked>false</masked>
      <name>BasicToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.B</defaultValue>
      <description></description>
      <id>1d28b5f8-09a0-4e6f-93df-5a54bbb3d4ba</id>
      <masked>false</masked>
      <name>B</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.path_users</defaultValue>
      <description></description>
      <id>927c1079-dca5-467a-b458-3746b833a5c7</id>
      <masked>false</masked>
      <name>path</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
