<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-user Id - 404</name>
   <tag></tag>
   <elementGuidId>1eca3f27-939d-4877-be4f-f59e8cd1294d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_id</name>
      <type>Main</type>
      <value>${client_id}</value>
      <webElementGuid>cc675733-1c65-4cc8-bf6e-835fe716e98a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_secret</name>
      <type>Main</type>
      <value>${client_secret}</value>
      <webElementGuid>2320047b-362c-43a9-85e7-a809555aefc2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${B} ${BasicToken}</value>
      <webElementGuid>c2d11b86-d72b-4d74-acd0-f31fec9e2ced</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${torei-db-sys-name}/${path}/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>335b69f1-aa32-4b7f-9ac0-474e33314fca</id>
      <name>schema 404 not found</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/Get user 404.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.client_id</defaultValue>
      <description></description>
      <id>ecc00c1d-00d2-47b0-b936-2a0a72f56490</id>
      <masked>false</masked>
      <name>client_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_secret</defaultValue>
      <description></description>
      <id>a6f02e76-1bbd-4e8f-aaaf-739b552dcfa8</id>
      <masked>false</masked>
      <name>client_secret</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.toreidbsysname</defaultValue>
      <description></description>
      <id>2ed27ebb-92f7-429d-a52b-48993fbe6cd2</id>
      <masked>false</masked>
      <name>torei-db-sys-name</name>
   </variables>
   <variables>
      <defaultValue>&quot;${GlobalVariable.username}:${GlobalVariable.password}&quot;.bytes.encodeBase64().toString()</defaultValue>
      <description></description>
      <id>ed83d222-564e-4caf-9e66-5f4174547653</id>
      <masked>false</masked>
      <name>BasicToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.B</defaultValue>
      <description></description>
      <id>13ca570f-5473-4e9e-a846-915bdd5380f1</id>
      <masked>false</masked>
      <name>B</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.path_users</defaultValue>
      <description></description>
      <id>d50765a7-b409-4436-99c1-81035c476a4b</id>
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
