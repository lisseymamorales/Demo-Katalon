import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testdata.InternalData as InternalData

WebUI.callTestCase(findTestCase('004 Cines Unidos/TC01 Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Cines Unidos/a_MI CUENTA'))

WebUI.click(findTestObject('Object Repository/Page_Cines Unidos/a_Actualizar Informacin'))

WebUI.navigateToUrl('https://www.cinesunidos.com/Mi_Cuenta/UpdateUser?hash=bGlzc2V5bWFtb3JhbGVzQGdtYWlsLmNvbQ==')

WebUI.click(findTestObject('Object Repository/Page_Cines Unidos/a_CONTRASEA'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Cines Unidos/input_CONTRASEA_Password'), Pass2)

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Cines Unidos/input_REPETIR CONTRASEA_PasswordComparation'), 
    Pass2)

GlobalVariable.passwordC = Pass2

WebUI.click(findTestObject('Object Repository/Page_Cines Unidos/button_Enviar'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Cines Unidos/p_El usuario se ha modificado exitosamente'), 
    'El usuario se ha modificado exitosamente')

WebUI.callTestCase(findTestCase('004 Cines Unidos/TC01 Login'), [('username') : GlobalVariable.usernameC, ('password') : GlobalVariable.passwordC], 
    FailureHandling.STOP_ON_FAILURE)

