<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Subtract</name>
   <tag></tag>
   <elementGuidId>194c88dc-90bb-4dba-9d7d-d5c19c73ce3e</elementGuidId>
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
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/Subtract</value>
      <webElementGuid>604c7c67-815c-4aa7-9dd2-ce1468a8f31b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>3c2b1b0b-c072-4a06-a0bb-3d77ddbeb8f5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:Subtract>
         &lt;tem:intA>3&lt;/tem:intA>
         &lt;tem:intB>3&lt;/tem:intB>
      &lt;/tem:Subtract>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://www.dneonline.com/calculator.asmx</soapServiceEndpoint>
   <soapServiceFunction>Subtract</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


String xmlPass =
&quot;&quot;&quot;&lt;?xml version=&quot;1.0&quot; encoding=&quot;utf-8&quot;?>
&lt;!-- Created with Liquid Technologies Online Tools 1.0 (https://www.liquid-technologies.com) -->
&lt;xs:schema xmlns:tns=&quot;http://tempuri.org/&quot; attributeFormDefault=&quot;unqualified&quot; elementFormDefault=&quot;qualified&quot; targetNamespace=&quot;http://tempuri.org/&quot; xmlns:xs=&quot;http://www.w3.org/2001/XMLSchema&quot;>
  &lt;xs:element name=&quot;SubtractResponse&quot;>
    &lt;xs:complexType>
      &lt;xs:sequence>
        &lt;xs:element name=&quot;SubtractResult&quot; type=&quot;xs:unsignedByte&quot; />
      &lt;/xs:sequence>
    &lt;/xs:complexType>
  &lt;/xs:element>
&lt;/xs:schema>
&quot;&quot;&quot;
boolean successful = WS.validateXmlAgainstSchema(response,xmlPass)</verificationScript>
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
