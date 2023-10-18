<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Patients - Post </name>
   <tag></tag>
   <elementGuidId>55ccaa81-ebd4-45f9-8212-ba677f45358b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \n    \&quot;firstName\&quot;: \&quot;${firstName}\&quot;,\n    \&quot;lastName\&quot;: \&quot;${lastName}\&quot;,\n    \&quot;address\&quot;: \&quot;${address}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;country\&quot;: \&quot;${country}\&quot;,\n    \&quot;gender\&quot;: \&quot;${gender}\&quot;,\n    \&quot;telephone\&quot;: \&quot;${telephone}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3712bdd0-729b-4305-ab7f-3e2c079403b4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_id</name>
      <type>Main</type>
      <value>10762030c38c414097032c32569af578</value>
      <webElementGuid>f3389110-3203-47f4-b55e-ae3203b73af9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_secret</name>
      <type>Main</type>
      <value>eac3527cED3f419B8FF56462D8BcF5Bd</value>
      <webElementGuid>ba93f3fd-720f-4e62-9961-407e1c4b6bb3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dG9yZWkxMjM6dG9yZWkxMjM=</value>
      <webElementGuid>02cceb2b-157f-4ba2-b090-fe717358b825</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://tr-patients-sys-api.us-e2.cloudhub.io/api/v1/patients</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'78788'</defaultValue>
      <description></description>
      <id>3e74fa16-27f1-4b64-987c-254dc98e8d2f</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>'Juarez'</defaultValue>
      <description></description>
      <id>c41fb634-a8a4-440e-b6d4-c97307c1168e</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>'Chacaito'</defaultValue>
      <description></description>
      <id>74ea6736-d51f-4024-96a6-4c3368eace22</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>'Caracas'</defaultValue>
      <description></description>
      <id>660baac3-5364-43e8-843c-72d907745206</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>'Venezuela'</defaultValue>
      <description></description>
      <id>a5fbddb4-b8cd-4777-a141-0306cd4f202b</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <variables>
      <defaultValue>'M'</defaultValue>
      <description></description>
      <id>13b05d2a-99d9-4eb9-9406-84427494d1fb</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>'0412178754'</defaultValue>
      <description></description>
      <id>a68c4366-e866-49bb-b208-1a5cfc88b795</id>
      <masked>false</masked>
      <name>telephone</name>
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





WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'firstName', '')
WS.verifyElementPropertyValue(response, 'codPatients', '')
WS.verifyElementPropertyValue(response, 'gender', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
