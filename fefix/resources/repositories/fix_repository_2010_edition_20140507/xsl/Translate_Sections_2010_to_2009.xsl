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

	<xsl:template match="Section">
		<Sections>
			<xsl:call-template name="translateEntityLevelAttributes2010to2009"><xsl:with-param name="cur" select="." /></xsl:call-template>			

			<Section><xsl:choose><xsl:when test="SectionID='PreTrade'">Pre Trade</xsl:when><xsl:when test="SectionID='PostTrade'">Post Trade</xsl:when><xsl:otherwise><xsl:value-of select="SectionID" /></xsl:otherwise></xsl:choose>
			</Section>	
			
			<DisplayName>
				<xsl:value-of select="Name" />
			</DisplayName>				

			<xsl:copy-of select="DisplayOrder" />		
			
			<Desc>
				<xsl:value-of select="Description" />
			</Desc>					

			<xsl:copy-of select="Volume" />	
			
			<NotReqXML><xsl:value-of select="NotReqXML" /></NotReqXML>
			
			<xsl:copy-of select="FIXMLFileName" />
		</Sections>
	</xsl:template>		

	<xsl:template match="Sections">
		<dataroot copyright="Copyright (c) FIX Protocol Ltd. All Rights Reserved." edition="2009" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"  xsi:noNamespaceSchemaLocation="../../schema/Sections.xsd">
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
