<?xml version="1.0" encoding="UTF-8"?>

<!-- 

Converts 2010 edition file to 2009 compatible file

Revisions
	01-Mar-2010		Phil Oliver
-->

<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
	<xsl:output method="xml" version="1.0" encoding="UTF-8" indent="yes"/>
	
	<xsl:include href="Translate_support.xsl" />

	<xsl:template match="comment()" >
		<xsl:copy/>
	</xsl:template>

	<xsl:template match="processing-instruction()">
	  <xsl:copy/>
	</xsl:template>

	<xsl:template match="Field">
		<Fields>
			<xsl:call-template name="translateEntityLevelAttributes2010to2009"><xsl:with-param name="cur" select="." /></xsl:call-template>			

			<xsl:copy-of select="Tag" />

			<FieldName>
				<xsl:copy-of select="Name/@*" />
				<xsl:value-of select="Name" />
			</FieldName>	
			
			<xsl:copy-of select="Type" />					

			<xsl:if test="Description">
				<Desc><xsl:value-of select="Description" /></Desc>
			</xsl:if>

			<xsl:if test="AssociatedDataTag">
				<LenRefers><xsl:value-of select="AssociatedDataTag" /></LenRefers>
			</xsl:if>
			
			<xsl:copy-of select="AbbrName" />	
			
			<xsl:if test="BaseCategory">
				<BaseCatagory><xsl:value-of select="BaseCategory" /></BaseCatagory>
			</xsl:if>
			
			<xsl:if test="BaseCategoryAbbrName">
				<BaseCatagoryXMLName><xsl:value-of select="BaseCategoryAbbrName" /></BaseCatagoryXMLName>
			</xsl:if>
			
			<xsl:copy-of select="NotReqXML" />	
			
			<xsl:if test="EnumDatatype">
				<UsesEnumsFromTag><xsl:value-of select="EnumDatatype" /></UsesEnumsFromTag>
			</xsl:if>			

			<xsl:copy-of select="UnionDataType" />
		</Fields>
	</xsl:template>		

	<xsl:template match="Fields">
		<dataroot copyright="Copyright (c) FIX Protocol Ltd. All Rights Reserved." edition="2009" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"  xsi:noNamespaceSchemaLocation="../../schema/Fields.xsd">
			<xsl:copy-of select="@version" />
			<xsl:copy-of select="@generated" />
			<xsl:if test="@latestEP"><xsl:attribute name="latestEP">EP<xsl:value-of select="@latestEP" /></xsl:attribute></xsl:if>
			<xsl:apply-templates />				
		</dataroot>
	</xsl:template>	
	
	<xsl:template match="/">
		<xsl:apply-templates />
	</xsl:template>
</xsl:stylesheet>
