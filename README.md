# ARXML

Autosar XML (`.arxml`) parser for Rust.

A current copy of the Autosar XSD must be obtained at [www.autosar.org](https://www.autosar.org/standards/classic-platform/classic-platform-440/) under "Methodology and Templates", the copy located in the repository is provided solely for informational, private use.


## Modifications xsd

Wrap the following in a comment, this otherwise currently leads to generation of two `Autosar` structs due to collisions with the definitions of the complexType "Autosar".

```xml
   <!-- <xsd:element name="AUTOSAR" type="AR:AUTOSAR">
      <xsd:annotation>
         <xsd:documentation>Root element of an AUTOSAR description, also the root element in corresponding XML documents.</xsd:documentation>
         <xsd:appinfo source="tags">mmt.qualifiedName="AUTOSAR";xml.globalElement="true"</xsd:appinfo>
         <xsd:appinfo source="stereotypes">atpObject</xsd:appinfo>
      </xsd:annotation>
   </xsd:element> -->
```