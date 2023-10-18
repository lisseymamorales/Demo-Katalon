<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-400 - Required</name>
   <tag></tag>
   <elementGuidId>d3314c44-abc2-43b3-8b5b-688fde22a93a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{    \n    \n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>${contentT}</value>
      <webElementGuid>71ff485a-4e25-4a8e-b410-b043b6a87ec5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_id</name>
      <type>Main</type>
      <value>${client_id}</value>
      <webElementGuid>5455d208-c2cc-4dd8-be6c-48420782c0d3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_secret</name>
      <type>Main</type>
      <value>${client_secret}</value>
      <webElementGuid>32e5ec99-400e-4203-bb27-f9477ed0e2f0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dG9yZWkxMjM6dG9yZWkxMjM=</value>
      <webElementGuid>91d76b42-a56b-4e0a-b7d2-3f785a2c4f3b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${torei-db-sys-name}${path}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>fc31c350-a47d-40f0-99b7-02541b4aa71a</id>
      <name>schema 400</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/Post User 400.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.client_id</defaultValue>
      <description></description>
      <id>ecd28e21-08aa-4021-844e-aa6f032e1b9e</id>
      <masked>false</masked>
      <name>client_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_secret</defaultValue>
      <description></description>
      <id>1437c300-b0f3-45bd-bb7c-501a2ea976a9</id>
      <masked>false</masked>
      <name>client_secret</name>
   </variables>
   <variables>
      <defaultValue>&quot;${GlobalVariable.username}:${GlobalVariable.password}&quot;.bytes.encodeBase64().toString()</defaultValue>
      <description></description>
      <id>7cc89dfb-c99f-48ba-8932-7df9fff51116</id>
      <masked>false</masked>
      <name>BasicToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.B</defaultValue>
      <description></description>
      <id>04b7e7ca-98ff-44ca-9982-3d12173e2f42</id>
      <masked>false</masked>
      <name>B</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.toreidbsysname</defaultValue>
      <description></description>
      <id>51eaeec5-5d0c-4e00-b732-357bb80a1e46</id>
      <masked>false</masked>
      <name>torei-db-sys-name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.path_users</defaultValue>
      <description></description>
      <id>a20ebe4a-26e1-4b08-a242-8318120b84dc</id>
      <masked>false</masked>
      <name>path</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.contentType</defaultValue>
      <description></description>
      <id>34860c3c-4476-412e-9884-25da7e949a7a</id>
      <masked>false</masked>
      <name>contentT</name>
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

WS.verifyResponseStatusCode(response, 400)

assertThat(response.getStatusCode()).isEqualTo(400)

WS.verifyElementPropertyValue(response, 'errorMessage', 'required key [CreditCard] not found\nrequired key [cOnline] not found\nrequired key [CDAccount] not found\nrequired key [SecuritiesAccount] not found\nrequired key [PersonalLoan] not found\nrequired key [Mortgage] not found\nrequired key [Education] not found\nrequired key [CCAvg] not found\nrequired key [Family] not found\nrequired key [ZIPCode] not found\nrequired key [Income] not found\nrequired key [Experience] not found\nrequired key [Age] not found')
WS.verifyElementPropertyValue(response, 'status', 'Error')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
