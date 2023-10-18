<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-200</name>
   <tag></tag>
   <elementGuidId>dc110b49-97ec-4f2e-9a6f-6062310b25f8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{    \n    \&quot;Age\&quot;: ${Age},\n    \&quot;Experience\&quot;: ${Experience},\n    \&quot;Income\&quot;: ${Income},\n    \&quot;ZIPCode\&quot;: \&quot;${ZIPCode}\&quot;,\n    \&quot;Family\&quot;: ${Family},\n    \&quot;CCAvg\&quot;: ${CCAvg},\n    \&quot;Education\&quot;: \&quot;${Education}\&quot;,\n    \&quot;Mortgage\&quot;: ${Mortgage},\n    \&quot;PersonalLoan\&quot;: ${PersonalLoan},\n    \&quot;SecuritiesAccount\&quot;: ${SecuritiesAccount},\n    \&quot;CDAccount\&quot;: ${CDAccount},\n    \&quot;cOnline\&quot;: ${cOnline},\n    \&quot;CreditCard\&quot;: \&quot;${CreditCard}\&quot;\n}&quot;,
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
      <webElementGuid>5b6ca0ff-d49d-4c56-9241-1e903a36912c</webElementGuid>
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
      <name>schema 200</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/Post User 200.json</data>
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
      <defaultValue>GlobalVariable.toreidbsysname</defaultValue>
      <description></description>
      <id>51eaeec5-5d0c-4e00-b732-357bb80a1e46</id>
      <masked>false</masked>
      <name>torei-db-sys-name</name>
   </variables>
   <variables>
      <defaultValue>25</defaultValue>
      <description>required</description>
      <id>6e6fc0fc-19ed-4529-8c1e-67614baf9f4c</id>
      <masked>false</masked>
      <name>Age</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description>required</description>
      <id>cd3072e1-d7c3-4736-bb73-0f0379faf6a5</id>
      <masked>false</masked>
      <name>Experience</name>
   </variables>
   <variables>
      <defaultValue>'100'</defaultValue>
      <description></description>
      <id>7563a149-02e2-4298-906f-68162927491d</id>
      <masked>false</masked>
      <name>Income</name>
   </variables>
   <variables>
      <defaultValue>'33130'</defaultValue>
      <description>required, min 5,max 5</description>
      <id>7a74f2bf-51db-41a1-8f8b-d6a8a2c77e53</id>
      <masked>false</masked>
      <name>ZIPCode</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>43df3dff-bcb6-4b3a-97c1-f09cee2057db</id>
      <masked>false</masked>
      <name>Family</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>00b80047-da0c-4ba8-8da0-2185cd98ef49</id>
      <masked>false</masked>
      <name>CCAvg</name>
   </variables>
   <variables>
      <defaultValue>'Superior'</defaultValue>
      <description></description>
      <id>16ff0b86-91fd-445d-8571-8429b2292b97</id>
      <masked>false</masked>
      <name>Education</name>
   </variables>
   <variables>
      <defaultValue>'500'</defaultValue>
      <description></description>
      <id>42a314fe-5238-4184-86a0-8ad31044845c</id>
      <masked>false</masked>
      <name>Mortgage</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>8d0264ea-34cb-413b-95f5-1e534ab0ee25</id>
      <masked>false</masked>
      <name>PersonalLoan</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>f97df952-fb78-4702-937e-49c9301be742</id>
      <masked>false</masked>
      <name>SecuritiesAccount</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>b5217518-d808-42b3-8580-1eb78cfdfd84</id>
      <masked>false</masked>
      <name>CDAccount</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>b66ca35d-a78f-41ab-bcbc-9438a9f93197</id>
      <masked>false</masked>
      <name>cOnline</name>
   </variables>
   <variables>
      <defaultValue>'378282246310000'</defaultValue>
      <description></description>
      <id>02b0b071-fe94-464c-bb1e-e5c5bd4725f5</id>
      <masked>false</masked>
      <name>CreditCard</name>
   </variables>
   <variables>
      <defaultValue>&quot;${GlobalVariable.username}:${GlobalVariable.password}&quot;.bytes.encodeBase64().toString()</defaultValue>
      <description></description>
      <id>7cc89dfb-c99f-48ba-8932-7df9fff51116</id>
      <masked>false</masked>
      <name>BasicToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.path_users</defaultValue>
      <description></description>
      <id>d69bbe2c-b3d6-4d8f-87db-b5f48cfb000f</id>
      <masked>false</masked>
      <name>path</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.B</defaultValue>
      <description></description>
      <id>b6d7f762-50e3-4b58-b4f1-2531ccbce65e</id>
      <masked>false</masked>
      <name>b</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.contentType</defaultValue>
      <description></description>
      <id>9d4f3518-1923-43ee-bd62-873b4392384a</id>
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




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
